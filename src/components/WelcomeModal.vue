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
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Welcome to Hedit</h2>
            <button
              class="p-1 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-gray-200 dark:hover:bg-zinc-700"
              @click="emit('close')"
            >
              <X class="w-5 h-5" />
            </button>
          </div>

          <div class="mt-4 text-sm text-gray-600 dark:text-gray-400">
            <p class="mb-3">
              Hedit is a fast, open-source hosts file editor for managing local, remote, and
              combined configurations.
            </p>
            <p>Create as many hosts files as you need and switch between them whenever you want.</p>
          </div>

          <div class="mt-8 flex justify-end">
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-primary-600 border border-transparent rounded-md shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 flex items-center justify-center"
              @click="getStarted"
            >
              Get Started
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { X } from 'lucide-vue-next'
  import { settingsStore } from '../stores/settings'

  const emit = defineEmits(['close'])
  const props = defineProps<{ show: boolean }>()

  const getStarted = () => {
    settingsStore.setHasCompletedOnboarding(true)
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
