import { listen } from '@tauri-apps/api/event'
import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

const store = await load('settings.json', { autoSave: false })

listen('reload-settings', async () => {
  await store.reload()
  await settingsStore.load()
})

export const settingsStore = reactive({
  isDarkTheme: false,
  license: '',
  activationId: '',
  personalUseOnly: false,
  isActivated: false,
  async load() {
    this.isDarkTheme =
      (await store.get<string>('theme')) === 'dark' ||
      window.matchMedia('(prefers-color-scheme: dark)').matches
    this.license = (await store.get<string>('license')) || ''
    this.activationId = (await store.get<string>('activationId')) || ''
    this.personalUseOnly = (await store.get<boolean>('personalUseOnly')) || false
    this.isActivated = (await store.get<boolean>('isActivated')) || false
  },
  setTheme(isDark: boolean) {
    this.isDarkTheme = isDark
    this.save()
  },
  setPersonalUseOnly(personalUseOnly: boolean) {
    this.personalUseOnly = personalUseOnly
    this.save()
  },
  async save() {
    await store.set('theme', this.isDarkTheme ? 'dark' : 'light')
    await store.set('license', this.license)
    await store.set('activationId', this.activationId)
    await store.set('personalUseOnly', this.personalUseOnly)
    await store.set('isActivated', this.isActivated)
    await store.save()
  },
})
