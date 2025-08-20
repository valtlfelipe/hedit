<template>
  <transition name="fade">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
      data-tauri-drag-region
      @click.self="$emit('close')"
    >
      <transition name="slide-up">
        <div v-if="show" class="w-full max-w-md p-6 bg-white rounded-lg shadow-xl dark:bg-zinc-800">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Rename File</h2>
          <div class="mt-4">
            <label for="filename" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Filename</label>
            <input
              ref="filenameInput"
              v-model="newName"
              type="text"
              class="block w-full px-3 py-2 mt-1 text-gray-900 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-white focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
              @keyup.enter="save"
            />
          </div>
          <div class="mt-6 flex justify-end space-x-3">
            <button
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500"
              @click="$emit('close')"
            >
              Cancel
            </button>
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-purple-600 border border-transparent rounded-md shadow-sm hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500"
              @click="save"
            >
              Save
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'

const props = defineProps<{
  show: boolean
  currentName: string
}>()

const emit = defineEmits<{
  close: []
  save: [newName: string]
}>()

const filenameInput = ref<HTMLInputElement | null>(null)

const newName = ref(props.currentName)

watch(
  () => props.show,
  (show) => {
    if (show) {
      newName.value = props.currentName
      nextTick(() => {
        filenameInput.value?.focus()
      })
    }
  },
)

function save() {
  if (newName.value.trim()) {
    emit('save', newName.value.trim())
  }
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.2s ease-out;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(20px);
}
</style>
