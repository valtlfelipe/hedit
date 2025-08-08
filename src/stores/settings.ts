import { reactive } from 'vue'

export const settingsStore = reactive({
  isDarkTheme: false,
  set(isDark: boolean) {
    this.isDarkTheme = isDark
  },
})
