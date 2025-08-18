import { watch } from 'vue'
import { settingsStore } from '../stores/settings'

export function useTheme() {
  const applyTheme = (isDark: boolean) => {
    if (isDark) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }

  const initializeTheme = () => {
    watch(
      () => settingsStore.isDarkTheme,
      (isDark) => {
        applyTheme(isDark)
      },
      { immediate: true },
    )
  }

  return {
    applyTheme,
    initializeTheme,
  }
}
