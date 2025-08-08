<template>
  <div class="w-64 bg-gray-50 dark:bg-gray-700 border-r border-gray-200 dark:border-gray-600 flex flex-col justify-between pb-25">
    <!-- Sidebar Header -->
    <div class="px-4 py-3 border-b border-gray-200 dark:border-gray-600">
      <h2 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">Local</h2>
    </div>

    <!-- File List -->
    <div class="overflow-y-auto flex-grow">
      <div class="p-2">
        <button
          v-for="file in files"
          :key="file.id"
          @click="$emit('fileSelect', file.id)"
          :class="[
            'w-full text-left px-3 py-2 rounded-lg flex items-center space-x-2 transition-colors',
            file.isSelected
              ? 'bg-blue-100 text-blue-800 dark:bg-gray-600 dark:text-gray-200'
              : 'text-gray-700 hover:bg-gray-200 dark:text-gray-200 dark:hover:bg-gray-600',
          ]"
        >
          <File class="w-4 h-4 text-gray-500 dark:text-gray-400" />
          <span class="text-sm flex-1">{{ file.name }}</span>
          <Check v-if="file.isActive" class="w-4 h-4 text-green-600 dark:text-green-400" />
        </button>
      </div>
    </div>

    <!-- Sidebar Footer -->
    <div class="px-4 py-2 border-t border-gray-200 dark:border-gray-600 bg-gray-100 dark:bg-gray-600">
      <div class="flex items-center space-x-2">
        <Folder class="w-4 h-4 text-gray-500 dark:text-gray-400" />
        <span class="text-xs text-gray-600 dark:text-gray-400">{{ statusText }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Check, File, Folder } from 'lucide-vue-next'
import { computed } from 'vue'

export interface FileItem {
  id: string
  name: string
  isActive: boolean
  isSelected: boolean
}

interface Props {
  files: FileItem[]
  status: string
}

const props = defineProps<Props>()

defineEmits<{
  fileSelect: [fileId: string]
}>()

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
</script>