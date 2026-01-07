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
              <X class="w-5 h-5"/>
            </button>
          </div>

          <div class="mt-4 text-sm text-gray-600 dark:text-gray-400">
            <p class="mb-3">
              Hedit is a powerful hosts file editor for macOS. Get started with the Free version or
              upgrade to Pro for more features.
            </p>
          </div>

          <div class="mt-6 space-y-4">
            <div class="flex items-start">
              <div class="flex items-center h-5">
                <CheckCircle2
                  class="w-5 h-5 text-primary-600 dark:text-primary-400 mr-3 flex-shrink-0"
                />
              </div>
              <div class="text-sm">
                <p class="font-medium text-gray-700 dark:text-gray-300">Free Version</p>
                <p class="text-gray-600 dark:text-gray-400">1 hosts file, personal use only</p>
              </div>
            </div>

            <div class="flex items-start">
              <div class="flex items-center h-5">
                <Zap class="w-5 h-5 text-primary-600 dark:text-primary-400 mr-3 flex-shrink-0"/>
              </div>
              <div class="text-sm">
                <p class="font-medium text-gray-700 dark:text-gray-300">
                  Pro Version ($25/license)
                </p>
                <p class="text-gray-600 dark:text-gray-400">Unlimited files, use forever</p>
              </div>
            </div>
          </div>

          <div class="mt-8 flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-primary-600 border border-transparent rounded-md shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 flex items-center justify-center"
              @click="startFree"
            >
              Start with Free
            </button>
            <button
              class="px-4 py-2 text-sm font-medium text-primary-600 bg-white border border-primary-600 rounded-md shadow-sm hover:bg-primary-50 dark:bg-zinc-700 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              @click="upgradeToPro"
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { X, CheckCircle2, Zap } from 'lucide-vue-next'
  import { settingsStore } from '../stores/settings'
  import { openUrl } from '@tauri-apps/plugin-opener'

  const emit = defineEmits(['close'])
  const props = defineProps<{ show: boolean }>()

  const startFree = () => {
    settingsStore.setHasCompletedOnboarding(true)
    settingsStore.setLicenseType('FREE')
    emit('close')
  }

  const upgradeToPro = () => {
    settingsStore.setHasCompletedOnboarding(true)
    settingsStore.setLicenseType('FREE')
    openUrl('https://hedit.app/pricing?ref=welcome_modal')
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
