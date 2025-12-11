<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">License</h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
        {{ settingsStore.isActivated ? 'Your license is active. Thank you for supporting Hedit!' : 'Activate your license to support Hedit.' }}
      </p>

      <button
        @click="openLicenseModal"
        class="flex items-center justify-center gap-2 px-4 py-2 text-sm text-white bg-purple-600 hover:bg-purple-700 rounded-md transition-colors"
      >
        <span>{{ settingsStore.isActivated ? 'Manage License' : 'Activate License' }}</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { settingsStore } from '../../stores/settings'

  const emit = defineEmits<{
    showLicenseModal: []
  }>()

  const openLicenseModal = () => {
    emit('showLicenseModal')
  }
</script>
