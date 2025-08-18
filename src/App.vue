<template>
  <MacOSWindow title="Hedit">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar :allow-activate="!selectedFile?.isActive" @create-file="handleCreateFile" @save-file="handleSaveFile"
        @activate-file="handleActivateFile" />

      <div class="flex flex-1 min-h-0">
        <Sidebar :files="hostsStore.files" @file-select="handleFileSelect" :status="selectedFile?.status || ''" />

        <CodeEditor v-if="selectedFile?.content || selectedFile?.content === ''" v-model="selectedFile.content" class="flex-1 min-w-0" ref="codeEditor" />
      </div>
    </div>
  </MacOSWindow>
</template>

<script setup lang="ts">
/** biome-ignore-all lint/correctness/noUnusedImports: biomejs is bugged */
import { listen } from '@tauri-apps/api/event'
import { readTextFile } from '@tauri-apps/plugin-fs'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
// biome-ignore lint/style/useImportType: biomejs is bugged
import CodeEditor from './components/CodeEditor.vue'
import MacOSWindow from './components/MacOSWindow.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { hostsStore } from './stores/files'
import { settingsStore } from './stores/settings'

listen('license-invalid', async () => {
  // TODO:
  console.warn('License is invalid, do something')
})

const codeEditor = ref<InstanceType<typeof CodeEditor> | null>(null)

const selectedFile = computed(() => hostsStore.files.find((f) => f.isSelected))

async function resetStatus() {
  setTimeout(() => {
    if (!selectedFile.value) {
      return
    }

    selectedFile.value.status = ''
  }, 3000)
}

async function writeHostsFile(isActivating?: boolean) {
  if (!selectedFile.value) {
    return
  }

  selectedFile.value.status = isActivating ? 'activating' : 'saving'

  try {
    await hostsStore.saveContent(selectedFile.value.id)
    selectedFile.value.status = isActivating ? 'activated' : 'saved'
    resetStatus()
  } catch (error) {
    selectedFile.value.status = 'save_error'
    console.error(error)
  }
}

async function loadFiles() {
  await hostsStore.load()
  if (hostsStore.files.length === 0) {
    const content = await readTextFile('/etc/hosts')
    hostsStore.create('Original File', content, true)
  }
}

watch(
  () => settingsStore.isDarkTheme,
  (isDark) => {
    if (isDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  },
  { immediate: true },
)

let fileSelectedChanged = true

watch(
  () => selectedFile.value?.content,
  () => {
    if (!selectedFile.value) return
    if (fileSelectedChanged) {
      fileSelectedChanged = false
      return
    }
    selectedFile.value.status = 'modified'
  },
)

listen('new_file', async () => handleCreateFile())
listen('activate_file', async () => handleActivateFile())
listen('save_file', async () => handleSaveFile())
listen('zoom_reset', () => {
  document.body.style.zoom = '100%'
})
listen('zoom_in', () => handleZoomIn())
listen('zoom_out', () => handleZoomOut())

const handleZoomIn = () => {
  const currentZoom = parseInt(document.body.style.zoom.replace('%', '')) || 100
  document.body.style.zoom = `${currentZoom + 10}%`
}

const handleZoomOut = () => {
  const currentZoom = parseInt(document.body.style.zoom.replace('%', '')) || 100
  document.body.style.zoom = `${currentZoom - 10}%`
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault()
    handleSaveFile()
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'n') {
    e.preventDefault()
    handleCreateFile()
  } else if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'a') {
    e.preventDefault()
    handleActivateFile()
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === '-') {
    e.preventDefault()
    handleZoomOut()
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === '=') {
    e.preventDefault()
    handleZoomIn()
  } else if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === '0') {
    e.preventDefault()
    document.body.style.zoom = '100%'
  }
}

const handleFileSelect = (fileId: string) => {
  fileSelectedChanged = true
  hostsStore.setSelected(fileId)
  if (codeEditor.value) {
    codeEditor.value.focus()
  }
}

const handleCreateFile = async () => {
  const id = await hostsStore.create(`New File ${hostsStore.files.length}`, '')
  if (id) {
    handleFileSelect(id)
  }
}

const handleSaveFile = () => {
  if (selectedFile.value && codeEditor.value?.hasErrors) {
    selectedFile.value.status = 'syntax_error'
    return
  }
  writeHostsFile()
}

const handleActivateFile = () => {
  if (!selectedFile.value?.id || selectedFile.value.isActive) return
  hostsStore.setActive(selectedFile.value?.id)
  writeHostsFile(true)
}

onMounted(() => {
  settingsStore.load()
  loadFiles()
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>