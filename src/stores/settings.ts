import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

const store = await load('settings.json', { autoSave: 100 })

export const settingsStore = reactive({
  isDarkTheme: false,
  license: '',
  personalUseOnly: false,
  async load() {
    const values = await store.entries()
    const themeValue = values.find(([key]) => key === 'theme')?.[1] as string
    this.isDarkTheme = values.find(([key]) => key === 'theme')?.[1] === 'dark'
    if (themeValue) {
      this.setTheme(themeValue === 'dark')
    } else {
      this.setTheme(window.matchMedia('(prefers-color-scheme: dark)').matches)
    }

    this.license = (values.find(([key]) => key === 'license')?.[1] as string) || ''
    this.personalUseOnly =
      (values.find(([key]) => key === 'personalUseOnly')?.[1] as boolean) || false
  },
  setTheme(isDark: boolean) {
    this.isDarkTheme = isDark
    this.save()
  },
  setLicense(license: string) {
    this.license = license
    this.save()
  },
  setPersonalUseOnly(personalUseOnly: boolean) {
    this.personalUseOnly = personalUseOnly
    this.save()
  },
  save() {
    return Promise.all([
      store.set('theme', this.isDarkTheme ? 'dark' : 'light'),
      store.set('license', this.license),
      store.set('personalUseOnly', this.personalUseOnly),
    ])
  },
})
