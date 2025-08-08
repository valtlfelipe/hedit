<template>
  <div class="bg-gray-50 border-b border-gray-200 px-2 py-2">
    <div class="flex items-center space-x-2">
      <!-- <button
        @click="$emit('createFile')"
        class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors"
      >
        <Plus class="w-4 h-4" />
        <span>Create</span>
      </button> -->

      <!-- <button
        @click="$emit('removeFile')"
        class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors"
      >
        <Trash2 class="w-4 h-4" />
        <span>Remove</span>
      </button> -->

      <button
        @click="$emit('saveFile')"
        class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors"
      >
        <Save class="w-4 h-4" />
        <span>Save</span>
      </button>

      <!-- <button
        @click="$emit('activateFile')"
        class="flex items-center space-x-1 px-3 py-1.5 text-sm text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
      >
        <Play class="w-4 h-4" />
        <span>Activate</span>
      </button> -->

      <div class="flex-1"></div>

      <!-- View Options -->
      <!-- <button class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors">
        <span>View</span>
        <ChevronDown class="w-4 h-4" />
      </button> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { Save } from 'lucide-vue-next'

defineEmits<{
  createFile: []
  removeFile: []
  saveFile: []
  activateFile: []
}>()
</script>