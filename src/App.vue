<template>
  <Toaster
    richColors
    :theme="settingsStore.isDarkTheme ? 'dark' : 'light'"
    :closeButton="true"
    closeButtonPosition="top-right"
    :duration="10000"
  />
  <AppWindow :title="title">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar
        :allow-activate="!selectedFile?.isActive"
        :license-type="settingsStore.licenseType"
        @create-local-file="() => handleCreateFile()"
        @create-remote-file="() => handleCreateFile({ remote: true })"
        @save-file="() => handleSaveFile()"
        @activate-file="handleActivateFile"
        @open-settings-modal="showSettingsModal = true"
        @open-settings-modal-with-tab="handleOpenSettingsModalWithTab"
      />

      <div class="flex flex-1 min-h-0 h-full">
        <Sidebar
          ref="sidebarRef"
          :files="hostsStore.files"
          :status="selectedFile?.status || ''"
          @file-select="handleFileSelect"
          @activate-file="handleActivateFile"
          @create-file="handleCreateFile"
        />

        <Suspense>
          <template #fallback>
            <LoadingSpinner class="m-auto"/>
          </template>

          <MonacoEditor
            v-if="selectedFile?.content || selectedFile?.content === ''"
            ref="codeEditor"
            v-model="selectedFile.content"
            class="flex-1 min-w-0"
            :is-dark-theme="settingsStore.isDarkTheme"
            @validation-status="handleValidationStatus"
          />
          <LoadingSpinner v-else class="m-auto"/>
        </Suspense>
      </div>
    </div>
    <WelcomeModal :show="showWelcomeModal" @close="showWelcomeModal = false"/>
    <UpgradePromptModal
      :show="showUpgradePromptModal"
      :message="upgradePromptMessage"
      @close="showUpgradePromptModal = false"
    />
    <SettingsModal
      :show="showSettingsModal"
      :initial-tab="settingsModalInitialTab"
      @close="showSettingsModal = false"
    />
  </AppWindow>
</template>

<script setup lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import { defineAsyncComponent, onBeforeUnmount, onMounted, ref, watch } from 'vue'
  import AppWindow from './components/AppWindow.vue'
  import SettingsModal from './components/SettingsModal.vue'
  import WelcomeModal from './components/WelcomeModal.vue'
  import UpgradePromptModal from './components/UpgradePromptModal.vue'
  import LoadingSpinner from './components/LoadingSpinner.vue'
  import Sidebar from './components/Sidebar.vue'
  import Toolbar from './components/Toolbar.vue'

  import { useFileOperations } from './composables/useFileOperations'
  import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
  import { useTelemetry } from './composables/useTelemetry'
  import { useTheme } from './composables/useTheme'
  import { hostsStore } from './stores/files'
  import { settingsStore } from './stores/settings'
  import { Toaster } from 'vue-sonner'

  const MonacoEditor = defineAsyncComponent(() => import('./components/MonacoEditor.vue'))

  const { trackEvent } = useTelemetry()

  const title = ref('Hedit')
  const showSettingsModal = ref(false)
  const settingsModalInitialTab = ref<string | undefined>(undefined)
  const showWelcomeModal = ref(false)
  const showUpgradePromptModal = ref(false)
  const upgradePromptMessage = ref('')
  const isContentValid = ref(true)
  const sidebarRef = ref()

  // Initialize composables
  const fileOperations = useFileOperations()
  const { initializeTheme } = useTheme()

  const { selectedFile, handleReloadContent, setFileStatus } = fileOperations

  const codeEditor = ref<InstanceType<typeof MonacoEditor> | null>(null)

  const handleValidationStatus = (isValid: boolean) => {
    isContentValid.value = isValid
  }

  // Enhanced handlers with additional logic
  const handleFileSelect = (fileId: string) => {
    fileOperations.handleFileSelect(fileId)
  }

  const handleCreateFile = async ({ remote = false, fileName = '', remoteUrl = '' } = {}) => {
    // Check if user is in Free mode and trying to create a second file
    if (
      (!settingsStore.licenseType || settingsStore.licenseType === 'FREE') &&
      hostsStore.files.length >= 1
    ) {
      upgradePromptMessage.value =
        'Upgrade to Pro to create unlimited hosts files. You can currently only use 1 file in Free mode.'
      showUpgradePromptModal.value = true
      return
    }

    if (remote && !fileName && !remoteUrl) {
      // Open remote file modal instead of creating directly
      sidebarRef.value?.showRemoteFileModal()
      return
    }

    trackEvent(`create_${remote ? 'remote' : 'local'}_file`)

    const id = await fileOperations.handleCreateFile({ remote, fileName, remoteUrl })
    if (id) {
      handleFileSelect(id)
    }
  }

  const handleSaveFile = () => {
    fileOperations.handleSaveFile(!isContentValid.value)
  }

  const handleActivateFile = (id?: string) => {
    fileOperations.handleActivateFile(id)
  }

  const handleOpenSettingsModalWithTab = (tab: string) => {
    settingsModalInitialTab.value = tab
    showSettingsModal.value = true
  }

  // Initialize event listeners and watchers
  const keyboardShortcuts = useKeyboardShortcuts(
    handleCreateFile,
    handleSaveFile,
    handleActivateFile,
  )

  keyboardShortcuts.initializeEventListeners()
  initializeTheme()

  // Handle license invalid event
  listen('license-update', async (event) => {
    const type = event.payload as string
    if (type === 'wrong-build') {
      showUpgradePromptModal.value = true
      upgradePromptMessage.value =
        'Your license is not valid for this build of Hedit. You can continue using Hedit in Free mode.'
    } else if (type === 'expired') {
      showUpgradePromptModal.value = true
      upgradePromptMessage.value =
        'Your Pro license has expired. You can continue using Hedit normally, but you will not receive updates until you renew your license.'
    } else if (type === 'invalid') {
      title.value = 'Hedit (License Invalid)'
    }
  })

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
      showUpgradePromptModal.value = false
      showWelcomeModal.value = false
    }
  }

  // Initialize on mount
  onMounted(() => {
    settingsStore.load().then(() => {
      // Show welcome modal on first launch if onboarding not completed
      if (!settingsStore.hasCompletedOnboarding && settingsStore.licenseType === 'FREE') {
        showWelcomeModal.value = true
        title.value = 'Hedit'
      } else if (settingsStore.licenseType === 'FREE') {
        title.value = 'Hedit'
      } else if (settingsStore.licenseType === 'PRO_ACTIVE') {
        title.value = 'Hedit (Pro)'
      } else if (settingsStore.licenseType === 'PRO_EXPIRED') {
        title.value = 'Hedit (Pro - Updates Expired)'
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
