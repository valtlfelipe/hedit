import { computed, ref } from 'vue'
import { hostsStore } from '../stores/files'
import { toast } from 'vue-sonner'

export function useFileOperations() {
  const selectedFile = computed(() => hostsStore.files.find((f) => f.isSelected))
  const fileSelectedChanged = ref(true)

  const loadFiles = async () => {
    await hostsStore.init()
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
      console.error('Error creating file:', error)
      toast.error(`Error creating file`, {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const handleSaveFile = async (hasErrors: boolean) => {
    if (!selectedFile.value) return

    if (hasErrors) {
      selectedFile.value.status = 'syntax_error'
      return
    }

    try {
      await hostsStore.saveContent(selectedFile.value.id)
    } catch (error) {
      console.error(error)
      toast.error('Error saving file', {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const handleActivateFile = async (id?: string) => {
    if (!id) {
      id = selectedFile.value?.id
    }

    if (!id) return

    const file = hostsStore.files.find((f) => f.id === id)
    if (!file || file.isActive) return

    const currentActive = hostsStore.files.find((f) => f.isActive)

    hostsStore.setActive(id)
    try {
      await hostsStore.saveContent(id)
    } catch (error) {
      if (currentActive?.id) {
        hostsStore.setActive(currentActive.id)
      }
      console.error(error)
      toast.error('Error activating file', {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const handleDeleteFile = async (id: string) => {
    try {
      await hostsStore.deleteFile(id)

      // Select the first file if available
      if (hostsStore.files.length > 0) {
        hostsStore.setSelected(hostsStore.files[0].id)
      }
    } catch (error) {
      console.error('Error deleting file:', error)
      toast.error('Error deleting file', {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const handleRenameFile = (id: string, newName: string) => {
    hostsStore.renameFile(id, newName)
  }

  const handleRefreshFile = async (id: string) => {
    try {
      await hostsStore.refreshRemoteFile(id)
      toast.success('Remote file refreshed successfully')
    } catch (error) {
      console.error('Failed to refresh remote file:', error)
      toast.error('Error refreshing remote file', {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const handleReloadContent = async (id: string) => {
    try {
      await hostsStore.reloadContent(id)
    } catch (error) {
      console.error('Error reloading file content:', error)
      toast.error('Error reloading file content', {
        description: error instanceof Error ? error.message : String(error),
      })
    }
  }

  const setFileStatus = (id: string, status: string) => {
    const file = hostsStore.files.find((f) => f.id === id)
    if (file) {
      file.status = status
    }
  }

  return {
    selectedFile,
    fileSelectedChanged,
    loadFiles,
    handleFileSelect,
    handleCreateFile,
    handleSaveFile,
    handleActivateFile,
    handleDeleteFile,
    handleRenameFile,
    handleRefreshFile,
    handleReloadContent,
    setFileStatus,
  }
}
