<template>
  <transition name="fade">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
      data-tauri-drag-region
      @click.self="emit('close')"
    >
      <transition name="slide-up">
        <div v-if="show" class="w-full max-w-md p-6 bg-white rounded-lg shadow-xl dark:bg-zinc-800">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Upgrade to Pro</h2>
            <button
              class="p-1 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-gray-200 dark:hover:bg-zinc-700"
              @click="emit('close')"
            >
              <X class="w-5 h-5"/>
            </button>
          </div>

          <div class="mt-4 text-sm text-gray-600 dark:text-gray-400">
            <p class="mb-3">{{ message }}</p>
          </div>

          <div class="mt-6 flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-primary-600 border border-transparent rounded-md shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 flex items-center justify-center"
              @click="upgradeToPro"
            >
              Upgrade to Pro
            </button>
            <button
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              @click="emit('close')"
            >
              Not Now
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { X } from 'lucide-vue-next'
  import { openUrl } from '@tauri-apps/plugin-opener'

  const emit = defineEmits(['close'])
  const props = defineProps<{
    show: boolean
    message: string
  }>()

  const upgradeToPro = () => {
    openUrl('https://hedit.app/pricing?ref=upgrade_prompt')
    emit('close')
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
