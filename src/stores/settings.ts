import { listen } from '@tauri-apps/api/event'
import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

const store = await load('settings.json', { autoSave: false, defaults: {} })

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
  autoUpdateHostsEnabled: false,
  autoUpdateHostsInterval: 24, // hours
  quitOnClose: false,
  autoStart: false,
  async load() {
    const preferredTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
    const savedTheme = await store.get<string>('theme')
    this.isDarkTheme = !savedTheme ? preferredTheme : savedTheme === 'dark'
    this.license = (await store.get<string>('license')) || ''
    this.activationId = (await store.get<string>('activationId')) || ''
    this.personalUseOnly = (await store.get<boolean>('personalUseOnly')) || false
    this.isActivated = (await store.get<boolean>('isActivated')) || false
    this.autoUpdateHostsEnabled = (await store.get<boolean>('autoUpdateHostsEnabled')) || false
    this.autoUpdateHostsInterval = (await store.get<number>('autoUpdateHostsInterval')) || 24
    this.quitOnClose = (await store.get<boolean>('quitOnClose')) || false
    this.autoStart = (await store.get<boolean>('autoStart')) || false
  },
  setTheme(isDark: boolean) {
    this.isDarkTheme = isDark
    this.save()
  },
  setPersonalUseOnly(personalUseOnly: boolean) {
    this.personalUseOnly = personalUseOnly
    this.save()
  },
  setAutoUpdateHosts(enabled: boolean, interval: number) {
    this.autoUpdateHostsEnabled = enabled
    this.autoUpdateHostsInterval = interval
    this.save()
  },
  setQuitOnClose(quitOnClose: boolean) {
    this.quitOnClose = quitOnClose
    this.save()
  },
  setAutoStart(autoStart: boolean) {
    this.autoStart = autoStart
    this.save()
  },
  async save() {
    await store.set('theme', this.isDarkTheme ? 'dark' : 'light')
    await store.set('license', this.license)
    await store.set('activationId', this.activationId)
    await store.set('personalUseOnly', this.personalUseOnly)
    await store.set('isActivated', this.isActivated)
    await store.set('autoUpdateHostsEnabled', this.autoUpdateHostsEnabled)
    await store.set('autoUpdateHostsInterval', this.autoUpdateHostsInterval)
    await store.set('quitOnClose', this.quitOnClose)
    await store.set('autoStart', this.autoStart)
    await store.save()
  },
})
