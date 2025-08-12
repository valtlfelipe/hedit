import { BaseDirectory, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { reactive, ref, watch } from 'vue'

export interface HostsFile {
  id: string
  name: string
  isActive: boolean
  isSelected: boolean
  content: string
  status: string
}

const metadataStore = await load('hosts-metadata.json', { autoSave: 100 })

async function writeProfileContent(id: string, content: string) {
  // await mkdir(`profiles`, { baseDir: BaseDirectory.AppData })
  await writeTextFile(`profiles/${id}.hosts`, content, {
    baseDir: BaseDirectory.AppData,
  })
}

export const hostsStore = reactive({
  files: [] as HostsFile[],
  setSelected(id: string) {
    this.files = this.files.map((file) =>
      file.id === id ? { ...file, isSelected: true } : { ...file, isSelected: false },
    )
  },
  setActive(id: string) {
    this.files = this.files.map((file) =>
      file.id === id ? { ...file, isActive: true } : { ...file, isActive: false },
    )
  },
  async create(name: string, content: string, isFirst?: boolean): Promise<string> {
    const id = crypto.randomUUID()
    const profile: HostsFile = {
      id,
      name,
      isActive: !!isFirst,
      isSelected: !!isFirst,
      content,
      status: '',
    }

    await writeProfileContent(id, content)

    this.files = [...this.files, profile]
    await metadataStore.set(
      'profiles',
      this.files.map(({ id, name, isActive, isSelected }) => ({ id, name, isActive, isSelected })),
    )

    return id
  },
  async load() {
    const profilesData = await metadataStore.get<HostsFile[]>('profiles')
    if (profilesData) {
      this.files = profilesData
    }
    await Promise.all(
      this.files.map(async (file) => {
        file.content = await readTextFile(`profiles/${file.id}.hosts`, {
          baseDir: BaseDirectory.AppData,
        })
      }),
    )
  },
})

// export const useProfileStore = defineStore('profiles', () => {
//   const profiles = ref<HostsFile[]>([])
//   const currentProfile = ref<HostsFile | null>(null)
//   const currentContent = ref('')

//   watch(currentProfile, async (profile) => {
//     if (profile) {
//       const content = await readProfileContent(profile.id)
//       currentContent.value = content
//     }
//   })

//   async function readProfileContent(id: string) {
//     const profilePath = `${await appLocalDataDir()}profiles/${id}.hosts`
//     return await readTextFile(profilePath)
//   }

//   async function writeProfileContent(id: string, content: string) {
//     const profilePath = `${await appLocalDataDir()}profiles/${id}.hosts`
//     await writeTextFile(profilePath, content)
//   }

//   async function loadProfiles() {
//     const profilesData = await store.get<HostsFile[]>('profiles')

//     if (!profilesData) {
//       return await initializeFirstProfile()
//     }

//     profiles.value = profilesData
//     const activeProfile = profilesData.find((p) => p.active)
//     currentProfile.value = activeProfile || profilesData[0]
//   }

//   async function initializeFirstProfile() {
//     const profileDir = `${await appLocalDataDir()}profiles`
//     if (!(await exists(profileDir))) {
//       await createDir(profileDir)
//     }

//     const hostsContent = await readTextFile('/etc/hosts')
//     const id = crypto.randomUUID()
//     const profile: HostsFile = {
//       id,
//       name: 'Default',
//       active: true,
//     }

//     await writeProfileContent(id, hostsContent)

//     profiles.value = [profile]
//     currentProfile.value = profile
//     await store.set('profiles', profiles.value)
//     await store.save()
//   }

//   async function createProfile(name: string) {
//     const id = crypto.randomUUID()
//     const profile: HostsFile = {
//       id,
//       name,
//       active: false,
//     }

//     await writeProfileContent(id, '# New profile')

//     profiles.value.push(profile)
//     await store.set('profiles', profiles.value)
//     await store.save()

//     currentProfile.value = profile
//   }

//   async function activateProfile(profile: HostsFile) {
//     await writeProfileContent(profile.id, currentContent.value)

//     await invoke('apply_hosts_file', { contents: currentContent.value })

//     for (const p of profiles.value) {
//       p.active = p.id === profile.id
//     }

//     currentProfile.value = profile

//     await store.set('profiles', profiles.value)
//     await store.save()
//   }

//   async function saveCurrentProfile() {
//     if (currentProfile.value) {
//       await writeProfileContent(currentProfile.value.id, currentContent.value)
//     }
//   }

//   return {
//     profiles,
//     currentProfile,
//     currentContent,
//     loadProfiles,
//     createProfile,
//     activateProfile,
//     saveCurrentProfile,
//     readProfileContent,
//   }
// })
// ;('')
