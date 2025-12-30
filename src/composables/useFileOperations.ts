import { readTextFile } from '@tauri-apps/plugin-fs'
import { computed, ref } from 'vue'
import type { HostsFile } from '../stores/files'
import { hostsStore } from '../stores/files'
import { toast } from 'vue-sonner'

export function useFileOperations() {
  const selectedFile = computed(() => hostsStore.files.find((f) => f.isSelected))
  const fileSelectedChanged = ref(true)

  const resetStatus = (file: HostsFile | undefined = selectedFile.value) => {
    if (!file) return

    setTimeout(() => {
      file.status = ''
    }, 3000)
  }

  const writeHostsFile = async (isActivating?: boolean) => {
    if (!selectedFile.value) {
      return
    }

    selectedFile.value.status = isActivating ? 'activating' : 'saving'

    try {
      await hostsStore.saveContent(selectedFile.value.id)
      selectedFile.value.status = isActivating ? 'activated' : 'saved'
      resetStatus(selectedFile.value)
    } catch (error) {
      selectedFile.value.status = 'save_error'
      console.error(error)
      toast.error(`Error ${isActivating ? 'activating' : 'saving'} file`, {
        description: error as string,
      })
    }
  }

  const loadFiles = async () => {
    await hostsStore.load()
    if (hostsStore.files.length === 0) {
      const content = await readTextFile('/etc/hosts')
      hostsStore.create('Original File', content, true)
    }
  }

  const handleFileSelect = (fileId: string) => {
    fileSelectedChanged.value = true
    hostsStore.setSelected(fileId)
  }

  const handleCreateFile = async ({ remote = false, fileName = '', remoteUrl = '' } = {}) => {
    try {
      const name = fileName || `New ${remote ? 'Remote' : 'Local'} File ${hostsStore.files.length}`
      const id = await hostsStore.create(name, '', false, remote, remoteUrl)
      return id
    } catch (error) {
      if (selectedFile.value) {
        selectedFile.value.status = 'save_error'
      }
      console.error('Error creating file:', error)
      toast.error(`Error creating file`, {
        description: error as string,
      })
    }
  }

  const handleSaveFile = async (hasErrors: boolean) => {
    if (!selectedFile.value) return

    if (hasErrors) {
      selectedFile.value.status = 'syntax_error'
      return
    }
    await writeHostsFile()
  }

  const handleActivateFile = async (id?: string) => {
    if (!id) {
      id = selectedFile.value?.id
    }

    if (!id) return

    const file = hostsStore.files.find((f) => f.id === id)

    if (!file || file.isActive) return

    hostsStore.setActive(id)
    await writeHostsFile(true)
  }

  return {
    selectedFile,
    fileSelectedChanged,
    loadFiles,
    handleFileSelect,
    handleCreateFile,
    handleSaveFile,
    handleActivateFile,
    resetStatus,
  }
}
