import { platform } from '@tauri-apps/plugin-os'
import { ref, onMounted } from 'vue'

export function usePlatform() {
  const modifier = ref('⌘') // Default to macOS

  onMounted(async () => {
    const os = await platform()
    if (os !== 'macos') {
      modifier.value = 'Ctrl'
    }
  })

  return {
    modifier,
  }
}
