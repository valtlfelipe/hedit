<template>
  <Toaster
    rich-colors
    :theme="settingsStore.isDarkTheme ? 'dark' : 'light'"
    :close-button="true"
    close-button-position="top-right"
    :duration="10000"
  />
  <AppWindow title="Hedit">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar
        :allow-activate="!selectedFile?.isActive"
        @create-file="handleCreateFile"
        @save-file="() => handleSaveFile()"
        @activate-file="handleActivateFile"
        @open-settings-modal="showSettingsModal = true"
      />

      <div class="flex flex-1 min-h-0 h-full">
        <Sidebar
          :files="hostsStore.files"
          :status="selectedFile?.status || ''"
          @file-select="handleFileSelect"
          @activate-file="handleActivateFile"
          @create-file="handleCreateFile"
        />

        <Suspense>
          <template #fallback> <LoadingSpinner class="m-auto" /> </template>

          <MonacoEditor
            v-if="selectedFile?.content || selectedFile?.content === ''"
            ref="codeEditor"
            v-model="selectedFile.content"
            class="flex-1 min-w-0"
            :is-dark-theme="settingsStore.isDarkTheme"
            @validation-status="handleValidationStatus"
          />
          <LoadingSpinner v-else class="m-auto" />
        </Suspense>
      </div>
    </div>
    <WelcomeModal :show="showWelcomeModal" @close="showWelcomeModal = false" />
    <SettingsModal :show="showSettingsModal" @close="showSettingsModal = false" />
    <CreateFileModal
      :show="showCreateFileModal"
      @close="showCreateFileModal = false"
      @created="handleFileCreated"
    />
  </AppWindow>
</template>

<script setup lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import { defineAsyncComponent, onBeforeUnmount, onMounted, ref, watch } from 'vue'
  import AppWindow from './components/AppWindow.vue'
  import SettingsModal from './components/SettingsModal.vue'
  import WelcomeModal from './components/WelcomeModal.vue'
  import LoadingSpinner from './components/LoadingSpinner.vue'
  import Sidebar from './components/Sidebar.vue'
  import Toolbar from './components/Toolbar.vue'
  import CreateFileModal from './components/CreateFileModal.vue'

  import { useFileOperations } from './composables/useFileOperations'
  import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
  import { useTheme } from './composables/useTheme'
  import { hostsStore } from './stores/files'
  import { settingsStore } from './stores/settings'
  import { Toaster } from 'vue-sonner'

  const MonacoEditor = defineAsyncComponent(() => import('./components/MonacoEditor.vue'))

  const showSettingsModal = ref(false)
  const showWelcomeModal = ref(false)
  const isContentValid = ref(true)
  const showCreateFileModal = ref(false)

  // Initialize composables
  const fileOperations = useFileOperations()
  const { initializeTheme } = useTheme()

  const { selectedFile, handleReloadContent, setFileStatus } = fileOperations

  const codeEditor = ref<InstanceType<typeof MonacoEditor> | null>(null)

  const handleValidationStatus = (isValid: boolean) => {
    isContentValid.value = isValid
  }

  const handleFileSelect = (fileId: string) => {
    fileOperations.handleFileSelect(fileId)
  }

  const handleCreateFile = () => {
    showCreateFileModal.value = true
  }

  const handleFileCreated = (fileId: string) => {
    handleFileSelect(fileId)
    showCreateFileModal.value = false
  }

  const handleSaveFile = () => {
    fileOperations.handleSaveFile(!isContentValid.value)
  }

  const handleActivateFile = (id?: string) => {
    fileOperations.handleActivateFile(id)
  }

  // Initialize event listeners and watchers
  const keyboardShortcuts = useKeyboardShortcuts(
    handleCreateFile,
    handleSaveFile,
    handleActivateFile,
  )

  keyboardShortcuts.initializeEventListeners()
  initializeTheme()

  listen('open_settings', async () => {
    showSettingsModal.value = true
  })

  listen('remote-hosts-updated', (event) => {
    const id = event.payload as string
    handleReloadContent(id)
  })

  // Watch for file content changes
  let isProgrammaticChange = false

  // Function to mark the next change as programmatic (not user-initiated)
  const markProgrammaticChange = () => {
    isProgrammaticChange = true
  }

  // Watch for file content changes
  watch(
    () => selectedFile.value?.content,
    () => {
      if (!selectedFile.value) return

      // If this is a programmatic change, don't mark as modified
      if (isProgrammaticChange) {
        isProgrammaticChange = false
        return
      }

      // If file was just selected, don't mark as modified
      if (fileOperations.fileSelectedChanged.value) {
        fileOperations.fileSelectedChanged.value = false
        return
      }

      // Only mark as modified if it's a real user change
      setFileStatus(selectedFile.value.id, 'modified')
    },
  )

  // Watch for file selection changes
  watch(
    () => selectedFile.value?.id,
    () => {
      // When file changes, mark the next content update as programmatic
      markProgrammaticChange()
      codeEditor.value?.focus()
    },
  )

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      showSettingsModal.value = false
      showWelcomeModal.value = false
      showCreateFileModal.value = false
    }
  }

  // Initialize on mount
  onMounted(() => {
    settingsStore.load().then(() => {
      // Show welcome modal on first launch if onboarding not completed
      if (!settingsStore.hasCompletedOnboarding) {
        showWelcomeModal.value = true
      }
    })
    fileOperations.loadFiles().then(() => {
      // After loading files, mark the initial content load as programmatic
      if (selectedFile.value) {
        markProgrammaticChange()
      }
    })
    window.addEventListener('keydown', handleKeydown)
  })

  onBeforeUnmount(() => {
    window.removeEventListener('keydown', handleKeydown)
  })
</script>
