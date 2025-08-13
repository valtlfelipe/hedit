import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

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
  // TODO: await mkdir(`profiles`, { baseDir: BaseDirectory.AppData })
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
    this.saveMetadata()
  },
  setActive(id: string) {
    this.files = this.files.map((file) =>
      file.id === id ? { ...file, isActive: true } : { ...file, isActive: false },
    )
    this.saveMetadata()
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
    this.saveMetadata()

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
  renameFile(id: string, newName: string) {
    this.files = this.files.map((file) => (file.id === id ? { ...file, name: newName } : file))
    this.saveMetadata()
  },
  async deleteFile(id: string) {
    if (this.files.find((file) => file.id === id)?.isActive) {
      return
    }
    this.files = this.files.filter((file) => file.id !== id)
    this.saveMetadata()
  },
  saveMetadata() {
    return metadataStore.set(
      'profiles',
      this.files.map(({ id, name, isActive, isSelected }) => ({ id, name, isActive, isSelected })),
    )
  },
})
