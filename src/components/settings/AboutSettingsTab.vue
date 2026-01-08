<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">About Hedit</h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4 italic">
        The modern hosts file editor for people who care about their tools.
      </p>
      <!-- biome-ignore format: biome is removing necessary spaces -->
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
        © {{ new Date(buildDate).getFullYear() }} FVM Tec. All rights reserved.
      </p>

      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">Version:</span>
          <span class="text-sm font-mono text-gray-800 dark:text-gray-200">{{ appVersion }}</span>
          <button
            v-if="!updateMessage"
            @click="checkForUpdates"
            :disabled="isCheckingUpdate"
            class="text-xs text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300 transition-colors hover:cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isCheckingUpdate ? 'Checking...' : 'Check for updates' }}
          </button>
          <a
            v-else-if="updateAvailable && downloadUrl"
            :href="downloadUrl"
            target="_blank"
            rel="noopener noreferrer"
            class="text-xs text-green-600 hover:text-green-700 dark:text-green-400 dark:hover:text-green-300 transition-colors hover:cursor-pointer underline"
          >
            {{ updateMessage }}
          </a>
          <span v-else class="text-xs text-gray-500 dark:text-gray-400"> {{ updateMessage }}</span>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">Build Date:</span>
          <span class="text-sm font-mono text-gray-800 dark:text-gray-200">{{ buildDate }}</span>
        </div>

        <hr class="my-4 border-t border-gray-200 dark:border-zinc-700">

        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">Feedback:</span>
          <button
            @click="openFeedbackLink"
            class="text-sm text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300 transition-colors hover:cursor-pointer"
          >
            Report an issue
          </button>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">Open Source:</span>
          <button
            @click="openSourceLink"
            class="text-sm text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300 transition-colors hover:cursor-pointer"
          >
            GitHub Repository
          </button>
        </div>
      </div>
    </div>

    <UpgradePromptModal
      :show="showUpgradePrompt"
      message="Your Pro license has expired. Upgrade to continue receiving updates and premium features."
      @close="showUpgradePrompt = false"
    />
  </div>
</template>

<script setup lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener'
  import { getVersion } from '@tauri-apps/api/app'
  import { invoke } from '@tauri-apps/api/core'
  import { onMounted, ref } from 'vue'
  import { settingsStore } from '../../stores/settings'
  import UpgradePromptModal from '../UpgradePromptModal.vue'

  interface UpdateInfo {
    available: boolean
    latest_version: string
    download_url: string
    current_version: string
  }

  const appVersion = ref('')
  const buildDate = ref('')
  const isCheckingUpdate = ref(false)
  const updateMessage = ref('')
  const updateAvailable = ref(false)
  const downloadUrl = ref('')
  const showUpgradePrompt = ref(false)

  onMounted(async () => {
    appVersion.value = await getVersion()
    buildDate.value = await invoke('get_build_date_command')
  })

  const checkForUpdates = async () => {
    // Check if license is expired and show upgrade prompt
    if (settingsStore.licenseType === 'PRO_EXPIRED') {
      showUpgradePrompt.value = true
      return
    }

    isCheckingUpdate.value = true
    updateMessage.value = ''
    downloadUrl.value = ''

    try {
      const updateInfo = await invoke<UpdateInfo>('check_for_updates_manual')

      if (updateInfo.available) {
        updateMessage.value = `Update available: ${updateInfo.latest_version}`
        updateAvailable.value = true
        downloadUrl.value = updateInfo.download_url
      } else {
        updateMessage.value = 'You are on the latest version'
        updateAvailable.value = false
      }
    } catch (error) {
      updateMessage.value = 'Failed to check for updates'
      updateAvailable.value = false
      console.error('Update check failed:', error)
    } finally {
      isCheckingUpdate.value = false
    }
  }

  const openFeedbackLink = () => {
    openUrl('https://github.com/valtlfelipe/hedit/issues/new/choose')
  }

  const openSourceLink = () => {
    openUrl('https://github.com/valtlfelipe/hedit')
  }
</script>
