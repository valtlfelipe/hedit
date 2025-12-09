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
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Activate License</h2>
            <button
              class="p-1 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-gray-200 dark:hover:bg-zinc-700"
              @click="emit('close')"
            >
              <X class="w-5 h-5"/>
            </button>
          </div>

          <div class="mt-4 text-sm text-gray-600 dark:text-gray-400">
            <p class="mb-3">
              If you are using this app for commercial use, you need to activate a license.
            </p>
            <div class="flex items-start p-3 bg-purple-50 rounded-lg dark:bg-purple-900/20">
              <Heart class="shrink-0 w-5 h-5 text-purple-600 dark:text-purple-400 mt-0.5"/>
              <p class="ml-3">
                Hedit is a fully indie project, and every license helps keep it that way. Thank you
                for your support!
              </p>
            </div>
          </div>

          <div class="mt-6">
            <div class="flex items-start">
              <div class="flex items-center h-5">
                <input
                  id="personal-use"
                  v-model="personalUseOnly"
                  :disabled="settingsStore.isActivated"
                  type="checkbox"
                  class="h-4 w-4 text-purple-600 border-gray-300 rounded dark:border-zinc-600 focus:ring-purple-500"
                  @change="onPersonalUseChange"
                >
              </div>
              <div class="ml-3 text-sm">
                <label for="personal-use" class="font-medium text-gray-700 dark:text-gray-300">
                  I am using this app for personal use only
                </label>
              </div>
            </div>

            <div v-if="!personalUseOnly" class="mt-6">
              <label
                for="license"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
              >
                License Key
              </label>
              <input
                id="license"
                v-model="license"
                :disabled="settingsStore.isActivated"
                :required="true"
                type="text"
                :class="['block w-full px-3 py-2 text-gray-900 bg-white rounded-md shadow-sm dark:bg-zinc-700 dark:text-white focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm border', error ? 'border-red-500 dark:border-red-500' : 'border-gray-300 dark:border-zinc-600']"
                placeholder="Enter your license key"
              >
              <div v-if="error" class="mt-2 text-sm text-red-600 dark:text-red-400">
                {{ errorText }}
              </div>
            </div>
          </div>

          <div class="mt-8 flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
            <a
              href="#"
              class="text-sm text-purple-600 hover:text-purple-800 hover:underline dark:hover:text-purple-400"
              @click.prevent="openPurchasePage()"
            >
              Purchase License
            </a>
            <div class="flex space-x-3">
              <button
                class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500"
                @click="emit('close')"
              >
                {{ settingsStore.isActivated || personalUseOnly ? 'Close' : 'Cancel' }}
              </button>
              <template v-if="!personalUseOnly">
                <button
                  v-if="!settingsStore.isActivated"
                  :disabled="isLoading"
                  class="px-4 py-2 text-sm font-medium text-white bg-purple-600 border border-transparent rounded-md shadow-sm hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 flex items-center justify-center min-w-[100px]"
                  @click="activate"
                >
                  <span v-if="isLoading" class="mr-2">
                    <svg
                      class="animate-spin h-4 w-4 text-white"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                    >
                      <circle
                        class="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        stroke-width="4"
                      ></circle>
                      <path
                        class="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
                      ></path>
                    </svg>
                  </span>
                  <span>{{ isLoading ? 'Activating...' : 'Activate' }}</span>
                </button>
                <button
                  v-else
                  disabled
                  class="px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 flex items-center justify-center"
                >
                  <svg class="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path
                      fill-rule="evenodd"
                      d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                      clip-rule="evenodd"
                    ></path>
                  </svg>
                  Activated!
                </button>
              </template>
            </div>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { openUrl } from '@tauri-apps/plugin-opener'
  import { Heart, X } from 'lucide-vue-next'
  import { ref, watch } from 'vue'
  import { settingsStore } from '../stores/settings'

  const emit = defineEmits(['close'])
  const props = defineProps<{ show: boolean }>()

  const license = ref(settingsStore.license)
  const personalUseOnly = ref(settingsStore.personalUseOnly)
  const error = ref(false)
  const errorText = ref('')
  const isLoading = ref(false)

  watch(
    () => props.show,
    (show) => {
      if (show) {
        personalUseOnly.value = settingsStore.personalUseOnly
        license.value = settingsStore.license
        error.value = false
        errorText.value = ''
      }
    },
  )

  function onPersonalUseChange() {
    settingsStore.setPersonalUseOnly(personalUseOnly.value)
  }

  async function activate() {
    error.value = false
    errorText.value = ''

    if (!license.value) {
      error.value = true
      errorText.value = 'License key is required.'
      return
    }

    isLoading.value = true

    try {
      await invoke('activate', { licenseKey: license.value })
    } catch (e) {
      console.error(e)
      error.value = true
      errorText.value = 'Activation failed. Please check your license key.'
    }

    isLoading.value = false
  }

  function openPurchasePage() {
    openUrl('https://hedit.app/pricing?ref=license_modal')
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
