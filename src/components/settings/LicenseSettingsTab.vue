<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-4">License</h3>

      <div v-if="settingsStore.licenseType === 'FREE'" class="mb-4">
        <div class="flex items-start gap-2">
          <div class="flex-shrink-0">
            <Info class="w-5 h-5 text-blue-500 dark:text-blue-400 mt-0.5"/>
          </div>
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              You are currently using Hedit in Free mode.
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Upgrade to Pro for unlimited files. Free mode is limited to 1 file.
            </p>
          </div>
        </div>
      </div>

      <div v-if="settingsStore.licenseType === 'PRO_ACTIVE'" class="mb-4">
        <div class="flex items-start gap-2">
          <div class="flex-shrink-0">
            <CheckCircle2 class="w-5 h-5 text-green-500 dark:text-green-400 mt-0.5"/>
          </div>
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Your Pro license is active. Thank you for supporting Hedit!
            </p>
          </div>
        </div>
      </div>

      <div v-if="settingsStore.licenseType === 'PRO_EXPIRED'" class="mb-4">
        <div class="flex items-start gap-2">
          <div class="flex-shrink-0">
            <AlertTriangle class="w-5 h-5 text-yellow-500 dark:text-yellow-400 mt-0.5"/>
          </div>
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">
              Your Pro license is expired, but you can continue using Hedit normally.
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Renew for 20% off to receive new features and improvements.
            </p>
          </div>
        </div>
      </div>

      <div v-if="settingsStore.licenseType !== 'FREE' && settingsStore.license" class="mb-4">
        <div class="flex items-center gap-2 p-4 bg-gray-50 dark:bg-zinc-700/50 rounded-md">
          <div>
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">License Key</p>
            <p class="text-sm text-gray-500 dark:text-gray-400 break-all">
              {{ settingsStore.license }}
            </p>
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mt-2">Valid Until</p>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {{ settingsStore.updateExpirationDate ? settingsStore.updateExpirationDate.split('T')[0] : 'N/A' }}
            </p>
          </div>
        </div>
      </div>

      <div v-if="settingsStore.licenseType === 'FREE'" class="space-y-3">
        <div>
          <label
            for="license"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >
            License Key
          </label>
          <input
            id="license"
            v-model="license"
            type="text"
            :class="['block w-full px-3 py-2 text-gray-900 bg-white rounded-md shadow-sm dark:bg-zinc-700 dark:text-white focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm border', error ? 'border-red-500 dark:border-red-500' : 'border-gray-300 dark:border-zinc-600']"
            placeholder="Enter your license key"
          >
          <div
            v-if="error"
            class="mt-2 text-sm text-red-600 dark:text-red-400 flex items-center gap-1"
          >
            <AlertCircle class="w-4 h-4"/>
            <span>{{ errorText }}</span>
          </div>
        </div>

        <button
          :disabled="isLoading"
          class="flex items-center justify-center gap-2 px-4 py-2 text-sm text-white bg-primary-600 hover:bg-primary-700 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
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
          <span>{{ isLoading ? 'Activating...' : 'Activate License' }}</span>
        </button>
      </div>

      <div v-else class="space-y-3">
        <button
          v-if="settingsStore.licenseType === 'PRO_EXPIRED'"
          class="flex items-center justify-center gap-2 px-4 py-2 text-sm text-primary-600 bg-white border border-primary-600 rounded-md shadow-sm hover:bg-primary-50 dark:bg-zinc-700 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          @click="openPurchasePage"
        >
          Renew License (20% off)
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'
  import { openUrl } from '@tauri-apps/plugin-opener'
  import { settingsStore } from '../../stores/settings'
  import { Info, CheckCircle2, AlertTriangle, AlertCircle } from 'lucide-vue-next'

  const license = ref(settingsStore.license)
  const error = ref(false)
  const errorText = ref('')
  const isLoading = ref(false)

  const openPurchasePage = () => {
    openUrl('https://hedit.app/pricing?ref=license_settings')
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
      // After successful activation, reload settings
      await settingsStore.load()
    } catch (e) {
      console.error(e)
      error.value = true
      errorText.value = 'Activation failed. Please check your license key.'
    }

    isLoading.value = false
  }
</script>
