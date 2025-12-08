<template>
  <div
    class="w-64 bg-gray-100 dark:bg-zinc-900 border-r border-gray-200 dark:border-zinc-800 flex flex-col justify-between"
  >
    <!-- Sidebar Header -->
    <!-- <div class="px-4 py-3 border-b border-gray-200 dark:border-zinc-800">
      <h2 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">Local</h2>
    </div> -->

    <!-- File List -->
    <div class="flex-grow" @contextmenu.prevent="showContextMenu($event)">
      <div class="p-2 space-y-1">
        <button
          v-for="file in files"
          :key="file.id"
          :class="[
            'w-full text-left px-3 py-2 rounded-lg flex items-center space-x-3 transition-colors',
            file.isSelected
              ? 'bg-purple-500/10 text-purple-700 dark:bg-purple-500/20 dark:text-purple-300'
              : 'text-gray-700 hover:bg-gray-200/80 dark:text-gray-300 dark:hover:bg-zinc-800/80',
          ]"
          @click="$emit('fileSelect', file.id)"
          @contextmenu.prevent="showFileContextMenu($event, file)"
        >
          <Tooltip v-if="!file.type || file.type === HostsFileType.LOCAL" text="Local File">
            <File class="w-4 h-4 text-gray-500 dark:text-gray-400" />
          </Tooltip>
          <Tooltip v-else-if="file.type === HostsFileType.REMOTE" text="Remote File">
            <Globe class="w-4 h-4 text-gray-500 dark:text-gray-400" />
          </Tooltip>
          <span class="text-sm font-medium flex-1 select-none truncate">{{ file.name }}</span>
          <Tooltip v-if="file.isActive" text="Current Active">
            <Play v-if="file.isActive" class="w-4 h-4 text-purple-700 dark:text-purple-300" />
          </Tooltip>
        </button>
      </div>
    </div>

    <!-- Sidebar Footer -->
    <div class="px-4 py-2 border-t border-gray-200 dark:border-zinc-800 bg-gray-100 dark:bg-zinc-900">
      <div class="flex items-center space-x-2">
        <Folder class="w-4 h-4 text-gray-500 dark:text-gray-400" />
        <span class="text-xs text-gray-600 dark:text-gray-400 font-medium">{{ statusText }}</span>
      </div>
    </div>
    <div ref="fileContextMenuContainer">
      <FileContextMenu
        v-if="fileContextMenu.show"
        :x="fileContextMenu.x"
        :y="fileContextMenu.y"
        :is-file-active="fileContextMenu.file?.isActive ?? false"
        @activate="activateFile"
        @edit="editFile"
        @delete="showConfirmModal"
        @click.stop
      />
    </div>
    <div ref="contextMenuContainer">
      <SidebarContextMenu
        v-if="contextMenu.show"
        :x="contextMenu.x"
        :y="contextMenu.y"
        @create-local="createFile"
        @create-remote="createFile({ remote: true })"
      />
    </div>
    <EditFileModal
      :show="editModal.show"
      :current-name="editModal.fileName"
      @close="hideEditModal"
      @save="saveNewName"
    />
    <ConfirmModal
      :show="confirmModal.show"
      :title="confirmModal.title"
      :message="confirmModal.message"
      @close="hideConfirmModal"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { File, Folder, Globe, Play } from 'lucide-vue-next'
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import type { HostsFile } from '../stores/files'
import { HostsFileType, hostsStore } from '../stores/files'
import ConfirmModal from './ConfirmModal.vue'
import FileContextMenu from './FileContextMenu.vue'
import EditFileModal from './EditFileModal.vue'
import Tooltip from './Tooltip.vue'
import SidebarContextMenu from './SidebarContextMenu.vue'

interface Props {
  files: HostsFile[]
  status: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  fileSelect: [fileId: string]
  activateFile: [fileId: string]
  createFile: [{ remote?: boolean }]
}>()

const contextMenuContainer = ref<HTMLElement | null>(null)
const fileContextMenuContainer = ref<HTMLElement | null>(null)

const fileContextMenu = reactive({
  show: false,
  x: 0,
  y: 0,
  file: null as HostsFile | null,
})

const contextMenu = reactive({
  show: false,
  x: 0,
  y: 0,
})

const editModal = reactive({
  show: false,
  fileId: '',
  fileName: '',
})

const confirmModal = reactive({
  show: false,
  title: '',
  message: '',
  fileId: '',
})

function activateFile() {
  if(!fileContextMenu.file) return
  emit('activateFile', fileContextMenu.file.id)
  hideContextMenu()
}

function createFile({ remote = false } = {}) {
  emit('createFile', { remote })
  hideContextMenu()
}

function showFileContextMenu(event: MouseEvent, file: HostsFile) {
  fileContextMenu.file = file
  fileContextMenu.x = event.clientX
  fileContextMenu.y = event.clientY
  fileContextMenu.show = true
}

function showContextMenu(event: MouseEvent) {
  if(fileContextMenu.show) return
  contextMenu.x = event.clientX
  contextMenu.y = event.clientY
  contextMenu.show = true
}

function hideContextMenu() {
  fileContextMenu.show = false
  contextMenu.show = false
}

function editFile() {
  if (fileContextMenu.file) {
    editModal.fileId = fileContextMenu.file.id
    editModal.fileName = fileContextMenu.file.name
    editModal.show = true
    hideContextMenu()
  }
}

function showConfirmModal() {
  if (fileContextMenu.file) {
    confirmModal.title = `Delete '${fileContextMenu.file.name}'`
    confirmModal.message = `Are you sure you want to delete '${fileContextMenu.file.name}'? This action cannot be undone.`
    confirmModal.fileId = fileContextMenu.file.id
    confirmModal.show = true
    hideContextMenu()
  }
}

function hideConfirmModal() {
  confirmModal.show = false
}

function confirmDelete() {
  hostsStore.deleteFile(confirmModal.fileId)
  hostsStore.setSelected(hostsStore.files[0]?.id)
  hideConfirmModal()
}

function hideEditModal() {
  editModal.show = false
}

function saveNewName(newName: string) {
  hostsStore.renameFile(editModal.fileId, newName)
  hideEditModal()
}

const statusText = computed(() => {
  switch (props.status) {
    case 'saving':
      return 'Saving...'
    case 'activating':
      return 'Activating...'
    case 'activated':
      return 'Activated successfully'
    case 'saved':
      return 'Saved successfully'
    case 'save_error':
      return 'Error occurred while saving'
    case 'fetching':
      return 'Fetching remote file...'
    case 'fetch_error':
      return 'Error fetching remote file'
    case 'syntax_error':
      return 'Not saved. Syntax error detected.'
    case 'loaded':
      return 'File loaded successfully'
    case 'modified':
      return 'File has been modified'
    default:
      return ''
  }
})

const handleClickOutside = (event: MouseEvent) => {
  if (
    (fileContextMenuContainer.value && !fileContextMenuContainer.value.contains(event.target as Node)) ||
    (contextMenuContainer.value && !contextMenuContainer.value.contains(event.target as Node))
  ) {
    hideContextMenu()
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    hideContextMenu()
    hideEditModal()
    hideConfirmModal()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('keydown', handleKeydown)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('keydown', handleKeydown)
})
</script>
