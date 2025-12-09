<template>
  <AppWindow :title="title">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar
        :allow-activate="!selectedFile?.isActive"
        @create-local-file="() => handleCreateFile()"
        @create-remote-file="() => handleCreateFile({ remote: true })"
        @save-file="() => handleSaveFile()"
        @activate-file="handleActivateFile"
        @show-license-modal="showLicenseModal = true"
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
            <LoadingSpinner class="m-auto" />
          </template>

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
    <LicenseModal :show="showLicenseModal" @close="handleLicenseModalClose" />
  </AppWindow>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { defineAsyncComponent, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import AppWindow from './components/AppWindow.vue'
import LicenseModal from './components/LicenseModal.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { useFileOperations } from './composables/useFileOperations'
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
import { useTheme } from './composables/useTheme'
import { hostsStore } from './stores/files'
import { settingsStore } from './stores/settings'
import LoadingSpinner from './components/LoadingSpinner.vue'

const MonacoEditor = defineAsyncComponent(() => import('./components/MonacoEditor.vue'))

const title = ref('Hedit')
const showLicenseModal = ref(false)
const isContentValid = ref(true)
const sidebarRef = ref()

// Initialize composables
const fileOperations = useFileOperations()
const { initializeTheme } = useTheme()

const { selectedFile } = fileOperations

const codeEditor = ref<InstanceType<typeof MonacoEditor> | null>(null)

const handleValidationStatus = (isValid: boolean) => {
  isContentValid.value = isValid
}

// Enhanced handlers with additional logic
const handleFileSelect = (fileId: string) => {
  fileOperations.handleFileSelect(fileId)
}

const handleCreateFile = async ({ remote = false, fileName = '', remoteUrl = '' } = {}) => {
  if (remote && !fileName && !remoteUrl) {
    // Open remote file modal instead of creating directly
    sidebarRef.value?.showRemoteFileModal()
    return
  }

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

// Initialize event listeners and watchers
const keyboardShortcuts = useKeyboardShortcuts(
  handleCreateFile,
  handleSaveFile,
  handleActivateFile,
)

keyboardShortcuts.initializeEventListeners()
initializeTheme()

const handleLicenseModalClose = () => {
  showLicenseModal.value = false
  if (
    !settingsStore.personalUseOnly &&
    !settingsStore.activationId
  ) {
    title.value = 'Hedit (Unlicensed)'
  } else if (settingsStore.personalUseOnly) {
    title.value = 'Hedit (Personal Use Only)'
  }
}

// Handle license invalid event
listen('license-invalid', async () => {
  console.warn('License is invalid, do something')
  title.value = 'Hedit (License Invalid)'
  showLicenseModal.value = true
})

listen('activate_license', async () => {
  showLicenseModal.value = true
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
    selectedFile.value.status = 'modified'
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
    showLicenseModal.value = false
  }
}

// Initialize on mount
onMounted(() => {
  settingsStore.load().then(() => {
    if (!settingsStore.personalUseOnly && !settingsStore.activationId) {
      showLicenseModal.value = true
      title.value = 'Hedit (Unlicensed)'
    } else if (settingsStore.personalUseOnly) {
      title.value = 'Hedit (Personal Use Only)'
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