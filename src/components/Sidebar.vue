<template>
  <div
    class="w-64 bg-gray-100 dark:bg-zinc-900 border-r border-gray-200 dark:border-zinc-800 flex flex-col justify-between"
  >
    <!-- Sidebar Header -->
    <div class="px-4 py-3 border-b border-gray-200 dark:border-zinc-800">
      <h2 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">Local</h2>
    </div>

    <!-- File List -->
    <div class="overflow-y-auto flex-grow">
      <div class="p-2 space-y-1">
        <button
          v-for="file in files"
          :key="file.id"
          @click="$emit('fileSelect', file.id)"
          @contextmenu.prevent="showContextMenu($event, file)"
          :class="[
            'w-full text-left px-3 py-2 rounded-lg flex items-center space-x-3 transition-colors',
            file.isSelected
              ? 'bg-blue-500/10 text-blue-700 dark:bg-blue-500/20 dark:text-blue-300'
              : 'text-gray-700 hover:bg-gray-200/80 dark:text-gray-300 dark:hover:bg-zinc-800/80',
          ]"
        >
          <File class="w-4 h-4 text-gray-500 dark:text-gray-400" />
          <span class="text-sm font-medium flex-1 select-none">{{ file.name }}</span>
          <Check v-if="file.isActive" class="w-4 h-4 text-green-500 dark:text-green-400" />
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
    <div ref="contextMenuContainer">
      <ContextMenu
        v-if="contextMenu.show"
        :x="contextMenu.x"
        :y="contextMenu.y"
        @edit="editFile"
        @delete="showConfirmModal"
        @click.stop
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
import { Check, File, Folder } from 'lucide-vue-next'
import { computed, onBeforeUnmount, onMounted, reactive, ref } from 'vue'
import type { HostsFile } from '../stores/files'
import { hostsStore } from '../stores/files'
import ConfirmModal from './ConfirmModal.vue'
import ContextMenu from './ContextMenu.vue'
import EditFileModal from './EditFileModal.vue'

interface Props {
  files: HostsFile[]
  status: string
}

const props = defineProps<Props>()

defineEmits<{
  fileSelect: [fileId: string]
}>()

const contextMenuContainer = ref<HTMLElement | null>(null)

const contextMenu = reactive({
  show: false,
  x: 0,
  y: 0,
  file: null as HostsFile | null,
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

function showContextMenu(event: MouseEvent, file: HostsFile) {
  contextMenu.file = file
  contextMenu.x = event.clientX
  contextMenu.y = event.clientY
  contextMenu.show = true
}

function hideContextMenu() {
  contextMenu.show = false
}

function editFile() {
  if (contextMenu.file) {
    editModal.fileId = contextMenu.file.id
    editModal.fileName = contextMenu.file.name
    editModal.show = true
    hideContextMenu()
  }
}

function showConfirmModal() {
  if (contextMenu.file) {
    confirmModal.title = `Delete ${contextMenu.file.name}`
    confirmModal.message = `Are you sure you want to delete ${contextMenu.file.name}? This action cannot be undone.`
    confirmModal.fileId = contextMenu.file.id
    confirmModal.show = true
    hideContextMenu()
  }
}

function hideConfirmModal() {
  confirmModal.show = false
}

function confirmDelete() {
  hostsStore.deleteFile(confirmModal.fileId)
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
    case 'saved':
      return 'Saved successfully'
    case 'error':
      return 'Error occurred while saving'
    case 'loaded':
      return 'File loaded successfully'
    case 'modified':
      return 'File has been modified'
    default:
      return ''
  }
})

const handleClickOutside = (event: MouseEvent) => {
  if (contextMenuContainer.value && !contextMenuContainer.value.contains(event.target as Node)) {
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
