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
import { readTextFile } from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
// biome-ignore lint/style/useImportType: biomejs is bugged
import CodeEditor from './components/CodeEditor.vue'
import MacOSWindow from './components/MacOSWindow.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { hostsStore } from './stores/files'
import { settingsStore } from './stores/settings'

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
    selectedFile.value.status = 'error'
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

async function setTheme() {
  const store = await load('settings.json', { autoSave: false })
  const val = await store.get<{ value: string }>('theme')
  settingsStore.set(val?.value === 'dark')
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

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault()
    handleSaveFile()
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
  writeHostsFile()
}

const handleActivateFile = () => {
  if (!selectedFile.value?.id) return
  hostsStore.setActive(selectedFile.value?.id)
  writeHostsFile(true)
}

onMounted(() => {
  setTheme()
  loadFiles()
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>