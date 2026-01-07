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
        </div>

        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">Build Date:</span>
          <span class="text-sm font-mono text-gray-800 dark:text-gray-200">{{ buildDate }}</span>
        </div>

        <!-- TODO: add option to check for updates -->

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
  </div>
</template>

<script setup lang="ts">
  import { openUrl } from '@tauri-apps/plugin-opener'
  import { getVersion } from '@tauri-apps/api/app'
  import { invoke } from '@tauri-apps/api/core'
  import { onMounted, ref } from 'vue'

  const appVersion = ref('')
  const buildDate = ref('')

  onMounted(async () => {
    appVersion.value = await getVersion()
    buildDate.value = await invoke('get_build_date_command')
  })

  const openFeedbackLink = () => {
    openUrl('https://github.com/valtlfelipe/hedit/issues/new/choose')
  }

  const openSourceLink = () => {
    openUrl('https://github.com/valtlfelipe/hedit')
  }
</script>
