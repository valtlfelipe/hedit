<template>
  <MacOSWindow title="Hedit">
    <div class="flex flex-col h-full flex-1 min-h-0">
      <Toolbar @create-file="handleCreateFile" @remove-file="handleRemoveFile" @save-file="handleSaveFile"
        @activate-file="handleActivateFile" />

      <div class="flex flex-1 min-h-0">
        <Sidebar :files="hostsStore.files" @file-select="handleFileSelect" :status="selectedFile?.status || ''" />

        <CodeEditor v-model="selectedFileContent" class="flex-1 min-w-0" />
      </div>
    </div>
  </MacOSWindow>
</template>

<script setup lang="ts">
/** biome-ignore-all lint/correctness/noUnusedImports: biomejs is bugged */
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import CodeEditor from './components/CodeEditor.vue'
import MacOSWindow from './components/MacOSWindow.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { hostsStore } from './stores/files'
import { settingsStore } from './stores/settings'

async function resetStatus() {
  if (!selectedFile.value) {
    return
  }

  setTimeout(() => {
    if (!selectedFile.value) {
      return
    }

    selectedFile.value.status = ''
  }, 3000)
}

async function writeHostsFile() {
  if (!selectedFile.value) {
    return
  }

  selectedFile.value.status = 'saving'

  try {
    await writeTextFile('/etc/hosts', selectedFile.value.content)
    selectedFile.value.status = 'saved'
    resetStatus()
    console.log('Hosts file saved successfully')
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
  selectedFileContent.value = hostsStore.files.find((f) => f.isSelected)?.content || ''
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
) // Ensure the theme is set immediately on mount

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault()
    handleSaveFile()
  }
}

const selectedFile = computed(() => hostsStore.files.find((f) => f.isSelected))
const selectedFileContent = ref('')

const handleFileSelect = (fileId: string) => {
  hostsStore.setSelected(fileId)
  selectedFileContent.value = selectedFile.value?.content || ''
}

const handleCreateFile = async () => {
  const id = await hostsStore.create(`New File ${hostsStore.files.length}`, '')
  if (id) {
    handleFileSelect(id)
  }
}

const handleRemoveFile = () => {
  // if (files.value.length > 1) {
  //   const updatedFiles = files.value.filter((f) => f.id !== selectedFileId.value)
  //   files.value = updatedFiles
  //   selectedFileId.value = updatedFiles[0]?.id || ''
  // }
}

const handleSaveFile = () => {
  console.log('Saving file:', selectedFile.value?.name)
  writeHostsFile()
}

const handleActivateFile = () => {
  if(!selectedFile.value?.id) return
  hostsStore.setActive(selectedFile.value?.id)
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