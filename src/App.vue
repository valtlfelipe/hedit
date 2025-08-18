<template>
  <MacOSWindow title="Hedit">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar
        :allow-activate="!selectedFile?.isActive"
        @create-file="handleCreateFile"
        @save-file="() => handleSaveFile()"
        @activate-file="handleActivateFile"
      />

      <div class="flex flex-1 min-h-0">
        <Sidebar
          :files="hostsStore.files"
          @file-select="handleFileSelect"
          :status="selectedFile?.status || ''"
        />

        <CodeEditor
          v-if="selectedFile?.content || selectedFile?.content === ''"
          v-model="selectedFile.content"
          class="flex-1 min-w-0"
          ref="codeEditor"
        />
      </div>
    </div>
  </MacOSWindow>
</template>

<script setup lang="ts">
/** biome-ignore-all lint/correctness/noUnusedImports: biomejs is bugged */
import { listen } from '@tauri-apps/api/event'
import { onMounted, ref, watch } from 'vue'
// biome-ignore lint/style/useImportType: biomejs is bugged
import CodeEditor from './components/CodeEditor.vue'
import MacOSWindow from './components/MacOSWindow.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { useFileOperations } from './composables/useFileOperations'
import { useFileWatcher } from './composables/useFileWatcher'
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts'
import { useTheme } from './composables/useTheme'
import { hostsStore } from './stores/files'
import { settingsStore } from './stores/settings'

// Initialize composables
const fileOperations = useFileOperations()
const { initializeTheme } = useTheme()
const fileWatcher = useFileWatcher()

const { selectedFile } = fileOperations

const codeEditor = ref<InstanceType<typeof CodeEditor> | null>(null)

// Enhanced handlers with additional logic
const handleFileSelect = (fileId: string) => {
  fileOperations.handleFileSelect(fileId)
  if (codeEditor.value) {
    codeEditor.value.focus()
  }
}

const handleCreateFile = async () => {
  const id = await fileOperations.handleCreateFile()
  if (id) {
    handleFileSelect(id)
  }
}

const handleSaveFile = () => {
  const hasErrors = selectedFile.value && codeEditor.value?.hasErrors
  fileOperations.handleSaveFile(!!hasErrors)
}

const handleActivateFile = () => {
  fileOperations.handleActivateFile()
}

// Initialize event listeners and watchers
const keyboardShortcuts = useKeyboardShortcuts(handleCreateFile, handleSaveFile, handleActivateFile)

keyboardShortcuts.initializeEventListeners()
initializeTheme()
fileWatcher.initializeFileWatcher(fileOperations.fileSelectedChanged)

// Handle license invalid event
listen('license-invalid', async () => {
  // TODO:
  console.warn('License is invalid, do something')
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
  },
)

// Initialize on mount
onMounted(() => {
  settingsStore.load()
  fileOperations.loadFiles().then(() => {
    // After loading files, mark the initial content load as programmatic
    if (selectedFile.value) {
      markProgrammaticChange()
    }
  })
})
</script>