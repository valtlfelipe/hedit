import { listen } from '@tauri-apps/api/event'
import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

const store = await load('settings.json', { autoSave: false })

listen('reload-settings', async () => {
  await store.reload()
  await settingsStore.load()
})

function getFromEntries<T>(values: [string, unknown][], key: string): T | undefined {
  return values.find(([k]) => k === key)?.[1] as T
}

export const settingsStore = reactive({
  isDarkTheme: false,
  license: '',
  activationId: '',
  personalUseOnly: false,
  isActivated: false,
  async load() {
    const values = await store.entries()

    const themeValue = getFromEntries<string>(values, 'theme')
    this.isDarkTheme = themeValue === 'dark'
    if (!themeValue) {
      this.isDarkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
    }

    this.license = getFromEntries<string>(values, 'license') || ''
    this.activationId = getFromEntries<string>(values, 'activationId') || ''
    this.personalUseOnly = getFromEntries<boolean>(values, 'personalUseOnly') || false
    this.isActivated = getFromEntries<boolean>(values, 'isActivated') || false
  },
  setTheme(isDark: boolean) {
    this.isDarkTheme = isDark
    this.save()
  },
  setPersonalUseOnly(personalUseOnly: boolean) {
    this.personalUseOnly = personalUseOnly
    console.log('Personal use only set to:', personalUseOnly)
    this.save()
  },
  async save() {
    await Promise.all([
      store.set('theme', this.isDarkTheme ? 'dark' : 'light'),
      store.set('license', this.license),
      store.set('activationId', this.activationId),
      store.set('personalUseOnly', this.personalUseOnly),
      store.set('isActivated', this.isActivated),
    ])
    return store.save()
  },
})
