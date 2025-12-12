import { invoke } from '@tauri-apps/api/core'
import {
  BaseDirectory,
  exists,
  mkdir,
  readTextFile,
  writeTextFile,
  remove,
} from '@tauri-apps/plugin-fs'
import { load } from '@tauri-apps/plugin-store'
import { reactive } from 'vue'

export enum HostsFileType {
  LOCAL = 'local',
  REMOTE = 'remote',
}

export interface HostsFile {
  id: string
  name: string
  isActive: boolean
  isSelected: boolean
  type: HostsFileType
  remoteUrl?: string | null
  content: string
  status: string
}

const metadataStore = await load('files-metadata.json', { autoSave: 100, defaults: { files: [] } })

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
  async create(
    name: string,
    content: string,
    isFirst?: boolean,
    remote?: boolean,
    remoteUrl?: string,
  ): Promise<string> {
    if (remote && !remoteUrl) {
      throw new Error('Remote URL is required for remote hosts file')
    }

    const id = crypto.randomUUID()
    const file: HostsFile = {
      id,
      name,
      isActive: !!isFirst,
      isSelected: !!isFirst,
      type: remote ? HostsFileType.REMOTE : HostsFileType.LOCAL,
      remoteUrl: remote ? remoteUrl : null,
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

    if (remote && file.remoteUrl) {
      await this.refreshRemoteFile(id)
    }

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
    await remove(`files/${id}.hosts`, { baseDir: BaseDirectory.AppData })
    this.saveMetadata()
  },
  async refreshRemoteFile(id: string) {
    const file = this.files.find((file) => file.id === id)
    if (!file || file.type !== HostsFileType.REMOTE || !file.remoteUrl) return

    try {
      file.status = 'fetching'
      await invoke('fetch_remote_hosts_file', { url: file.remoteUrl, fileName: `${id}.hosts` })

      // Reload the content after fetching
      file.content = await readTextFile(`files/${id}.hosts`, {
        baseDir: BaseDirectory.AppData,
      })
      file.status = 'loaded'

      // If the file is active, update the system hosts as well
      if (file.isActive) {
        await invoke('write_system_hosts', { content: file.content })
      }
    } catch (error) {
      file.status = 'fetch_error'
      throw error
    }
  },
  async saveContent(id: string) {
    const file = this.files.find((file) => file.id === id)
    if (!file) return

    await writeTextFile(`files/${file.id}.hosts`, file.content, {
      baseDir: BaseDirectory.AppData,
    })

    if (file.isActive) {
      await invoke('write_system_hosts', { content: file.content })
    }
  },
  async reloadContent(id: string) {
    const file = this.files.find((file) => file.id === id)
    if (!file) return

    file.content = await readTextFile(`files/${file.id}.hosts`, {
      baseDir: BaseDirectory.AppData,
    })
  },
  saveMetadata() {
    return metadataStore.set(
      'files',
      this.files.map(({ id, name, type, remoteUrl, isActive, isSelected }) => ({
        id,
        name,
        type,
        remoteUrl,
        isActive,
        isSelected,
      })),
    )
  },
})
