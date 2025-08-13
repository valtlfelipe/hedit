import { BaseDirectory, exists, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
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

const metadataStore = await load('files-metadata.json', { autoSave: 100 })

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
    const file: HostsFile = {
      id,
      name,
      isActive: !!isFirst,
      isSelected: !!isFirst,
      content,
      status: '',
    }

    const dirExists = await exists('files', {
      baseDir: BaseDirectory.AppData,
    })

    if (!dirExists) {
      await mkdir(`files`, { baseDir: BaseDirectory.AppData })
    }

    await writeTextFile(`files/${id}.hosts`, content, {
      baseDir: BaseDirectory.AppData,
    })

    this.files = [...this.files, file]
    this.saveMetadata()

    return id
  },
  async load() {
    const filesData = await metadataStore.get<HostsFile[]>('files')
    if (filesData) {
      this.files = filesData
    }
    await Promise.all(
      this.files.map(async (file) => {
        file.content = await readTextFile(`files/${file.id}.hosts`, {
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
  async saveContent(id: string) {
    const file = this.files.find((file) => file.id === id)
    if (!file) return

    await writeTextFile(`files/${file.id}.hosts`, file.content, {
      baseDir: BaseDirectory.AppData,
    })
    if (file.isActive) {
      await writeTextFile('/etc/hosts', file.content)
    }
  },
  saveMetadata() {
    return metadataStore.set(
      'files',
      this.files.map(({ id, name, isActive, isSelected }) => ({ id, name, isActive, isSelected })),
    )
  },
})
