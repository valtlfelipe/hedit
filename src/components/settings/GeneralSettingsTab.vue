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
  </div>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { settingsStore } from '../../stores/settings'
  import Switch from '../Switch.vue'

  const isDarkTheme = ref(settingsStore.isDarkTheme)

  const toggleDarkMode = async () => {
    settingsStore.setTheme(isDarkTheme.value)
  }

  // Watch for theme changes from the store
  watch(
    () => settingsStore.isDarkTheme,
    (newVal) => {
      isDarkTheme.value = newVal
    },
  )
</script>
