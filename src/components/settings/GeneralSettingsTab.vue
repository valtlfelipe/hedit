<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">Appearance</h3>
      <Switch
        id="darkModeToggle"
        v-model="isDarkTheme"
        :label="isDarkTheme ? 'Dark Mode' : 'Light Mode'"
        @change="toggleDarkMode"
      />
    </div>
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">Quit when closing window</h3>
      <Switch
        id="quitOnCloseToggle"
        v-model="isQuitOnClose"
        :label="isQuitOnClose ? 'Quit on Close Enabled' : 'Quit on Close Disabled'"
        @change="toggleQuitOnClose"
      />
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
        When disabled, allows the application to continue running in the background after closing the main window.
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { settingsStore } from '../../stores/settings'
  import Switch from '../Switch.vue'

  const isDarkTheme = ref(settingsStore.isDarkTheme)
  const isQuitOnClose = ref(settingsStore.quitOnClose)

  const toggleDarkMode = () => {
    settingsStore.setTheme(isDarkTheme.value)
  }

  const toggleQuitOnClose = () => {
    settingsStore.setQuitOnClose(isQuitOnClose.value)
  }

  // Watch for theme changes from the store
  watch(
    () => settingsStore.isDarkTheme,
    (newVal) => {
      isDarkTheme.value = newVal
    },
  )
</script>
