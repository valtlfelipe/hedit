<template>
  <transition name="fade">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
      @click.self="$emit('close')"
    >
      <transition name="slide-up">
        <div v-if="show" class="w-full max-w-md p-6 bg-white rounded-lg shadow-xl dark:bg-zinc-800">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Activate License</h2>
          <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            If you are using this app for commercial use, you need to activate a license.
          </p>
          <div class="mt-4">
            <label for="license" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              License Key
            </label>
            <input
              id="license"
              v-model="license"
              :disabled="personalUseOnly"
              type="text"
              class="block w-full px-3 py-2 mt-1 text-gray-900 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-white focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              placeholder="Enter your license key"
            />
          </div>
          <div class="flex items-center mt-4">
            <input
              id="personal-use"
              v-model="personalUseOnly"
              type="checkbox"
              class="h-4 w-4 text-blue-600 border-gray-300 rounded dark:border-zinc-600 focus:ring-blue-500"
            />
            <label for="personal-use" class="ml-2 block text-sm text-gray-900 dark:text-gray-300">
              I am using this app for personal use only
            </label>
          </div>
          <div class="mt-6 flex justify-end space-x-3">
            <button
              @click="$emit('close')"
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Cancel
            </button>
            <button
              @click="activateLicense"
              class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Activate
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { settingsStore } from '../stores/settings'

defineProps<{
  show: boolean
}>()

const license = ref(settingsStore.license)
const personalUseOnly = ref(settingsStore.personalUseOnly)

const emit = defineEmits(['close'])

function activateLicense() {
  settingsStore.setLicense(license.value)
  settingsStore.setPersonalUseOnly(personalUseOnly.value)
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
