import { listen } from '@tauri-apps/api/event'
import { onMounted, onUnmounted } from 'vue'
import { zoomUtils } from '../utils/zoomUtils'
import { useZoom } from './useZoom'

export function useKeyboardShortcuts(
  handleCreateFile: (options?: { remote?: boolean }) => Promise<void>,
  handleSaveFile: () => void,
  handleActivateFile: () => void,
) {
  const { handleZoomIn, handleZoomOut } = useZoom()

  const handleKeydown = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
      e.preventDefault()
      handleSaveFile()
    } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key.toLowerCase() === 'n') {
      e.preventDefault()
      handleCreateFile()
    } else if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'n') {
      e.preventDefault()
      handleCreateFile({ remote: true })
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
      zoomUtils.handleZoomReset()
    }
  }

  const initializeEventListeners = () => {
    onMounted(() => {
      window.addEventListener('keydown', handleKeydown)

      // Tauri event listeners
      listen('new_file', async () => handleCreateFile())
      listen('new_remote_file', async () => handleCreateFile({ remote: true }))
      listen('activate_file', async () => handleActivateFile())
      listen('save_file', async () => handleSaveFile())
      listen('zoom_reset', () => zoomUtils.handleZoomReset())
      listen('zoom_in', () => handleZoomIn())
      listen('zoom_out', () => handleZoomOut())
    })

    onUnmounted(() => {
      window.removeEventListener('keydown', handleKeydown)
    })
  }

  return {
    handleKeydown,
    initializeEventListeners,
  }
}
