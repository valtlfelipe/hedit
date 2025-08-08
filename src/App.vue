<template>
  <MacOSWindow title="Hedit">
    <div class="flex flex-col h-full flex-1">
      <Toolbar @create-file="handleCreateFile" @remove-file="handleRemoveFile" @save-file="handleSaveFile"
        @activate-file="handleActivateFile" />

      <div class="flex h-full flex-1">
        <Sidebar :files="files" @file-select="handleFileSelect" :status="selectedFile?.status || ''" />

        <CodeEditor :content="selectedFile?.content || ''" @change="handleContentChange" />
      </div>
    </div>
  </MacOSWindow>
</template>

<script setup lang="ts">
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import CodeEditor from './components/CodeEditor.vue'
import MacOSWindow from './components/MacOSWindow.vue'
import type { FileItem } from './components/Sidebar.vue'
import Sidebar from './components/Sidebar.vue'
import Toolbar from './components/Toolbar.vue'
import { settingsStore } from './stores/settings'

async function readHostsFile() {
  try {
    const content = await readTextFile('/etc/hosts')

    files.value.push({
      id: '1',
      name: 'Original File',
      isActive: true,
      isSelected: true,
      content: content as string,
      status: 'Loaded successfully',
    })
  } catch (error) {
    console.error(error)
  }
}

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

async function setTheme() {
  const store = await load('settings.json', { autoSave: false })
  const val = await store.get<{ value: string }>('theme')
  settingsStore.set(val?.value === 'dark')
}

watch(() => settingsStore.isDarkTheme, (isDark) => {
  if (isDark) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}, { immediate: true }) // Ensure the theme is set immediately on mount

onMounted(() => {
  readHostsFile()
  setTheme()
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault()
    handleSaveFile()
  }
}

interface FileItemWithContent extends FileItem {
  content: string
  status: string
}

const files = ref<FileItemWithContent[]>([])

const selectedFileId = ref('1')

const selectedFile = computed(() => files.value.find((f) => f.id === selectedFileId.value))

const handleFileSelect = (fileId: string) => {
  selectedFileId.value = fileId
  files.value = files.value.map((f) => ({
    ...f,
    isSelected: f.id === fileId,
  }))
}

const handleContentChange = (content: string) => {
  files.value = files.value.map((f) =>
    f.id === selectedFileId.value ? { ...f, content, status: 'modified' } : f,
  )
}

const handleCreateFile = () => {
  const newFile: FileItemWithContent = {
    id: Date.now().toString(),
    name: `New File ${files.value.length}`,
    isActive: false,
    isSelected: false,
    content: '',
    status: '',
  }
  files.value = [...files.value, newFile]
}

const handleRemoveFile = () => {
  if (files.value.length > 1) {
    const updatedFiles = files.value.filter((f) => f.id !== selectedFileId.value)
    files.value = updatedFiles
    selectedFileId.value = updatedFiles[0]?.id || ''
  }
}

const handleSaveFile = () => {
  console.log('Saving file:', selectedFile.value?.name)
  writeHostsFile()
}

const handleActivateFile = () => {
  files.value = files.value.map((f) => ({
    ...f,
    isActive: f.id === selectedFileId.value,
  }))
}
</script>