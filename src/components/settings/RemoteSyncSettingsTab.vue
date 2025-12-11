<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">Remote Sync</h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
        Automatically sync your remote hosts files at regular intervals.
      </p>

      <div class="space-y-4">
        <Switch
          id="autoUpdateEnabled"
          v-model="autoUpdateEnabled"
          label="Enable automatic sync"
          @change="updateAutoUpdateSettings"
        />

        <div v-if="autoUpdateEnabled" class="space-y-2">
          <label
            for="autoUpdateInterval"
            class="text-sm text-gray-600 dark:text-gray-400 block mb-1"
          >
            Sync interval (hours):
          </label>
          <div class="flex items-center gap-2">
            <input
              id="autoUpdateInterval"
              v-model.number="autoUpdateInterval"
              type="number"
              min="1"
              max="168"
              class="w-20 px-3 py-2 text-sm bg-gray-100 border border-gray-300 rounded focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200"
              :disabled="!autoUpdateEnabled"
              @change="updateAutoUpdateSettings"
            >
            <span class="text-sm text-gray-600 dark:text-gray-400">hours</span>
          </div>
        </div>

        <hr class="border-gray-200 dark:border-zinc-700">

        <div class="pt-2" v-if="autoUpdateEnabled">
          <button
            class="flex items-center justify-center gap-2 px-4 py-2 text-sm text-white bg-purple-600 hover:bg-purple-700 rounded-md transition-colors"
            @click="triggerManualSync"
            :disabled="!autoUpdateEnabled || isSyncing"
          >
            <RefreshCw class="w-4 h-4" :class="{'animate-spin': isSyncing}"/>
            <span>Sync Now</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Sync Status -->
    <div class="mt-6 p-3 bg-gray-100 dark:bg-gray-700 rounded-md" v-if="autoUpdateEnabled">
      <div class="flex items-center gap-2">
        <div
          class="w-2 h-2 rounded-full"
          :class="{
          'bg-green-500': syncStatus === 'success',
          'bg-yellow-500': syncStatus === 'in_progress',
          'bg-red-500': syncStatus === 'error',
          'bg-gray-400': syncStatus === 'idle'
        }"
        ></div>
        <span class="text-sm font-medium text-gray-800 dark:text-gray-200">Status:</span>
        <span class="text-sm capitalize text-gray-700 dark:text-gray-300"
          >{{ syncStatusText }}</span
        >
      </div>
      <div v-if="lastSyncTime" class="text-xs text-gray-600 dark:text-gray-400 mt-2">
        Last sync: {{ lastSyncTime }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed, ref } from 'vue'
  import { settingsStore } from '../../stores/settings'
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'
  import { RefreshCw } from 'lucide-vue-next'
  import Switch from '../Switch.vue'

  const autoUpdateEnabled = ref(settingsStore.autoUpdateHostsEnabled)
  const autoUpdateInterval = ref(settingsStore.autoUpdateHostsInterval)
  const syncStatus = ref<'idle' | 'in_progress' | 'success' | 'error'>('idle')
  const isSyncing = computed(() => syncStatus.value === 'in_progress')
  const syncStatusText = ref('Ready')
  const lastSyncTime = ref<string | null>(null)

  const updateAutoUpdateSettings = () => {
    // Validate interval
    if (autoUpdateEnabled.value) {
      if (
        Number.isNaN(autoUpdateInterval.value) ||
        autoUpdateInterval.value === null ||
        autoUpdateInterval.value === undefined
      ) {
        syncStatus.value = 'error'
        syncStatusText.value = 'Please enter a valid number'
        autoUpdateInterval.value = 24 // Reset to default
        setTimeout(() => {
          syncStatus.value = 'idle'
          syncStatusText.value = 'Ready'
        }, 3000)
        return
      }

      if (autoUpdateInterval.value < 1 || autoUpdateInterval.value > 168) {
        syncStatus.value = 'error'
        syncStatusText.value = `Interval must be between 1 and 168 hours`
        // Reset to default if invalid
        autoUpdateInterval.value = Math.min(Math.max(autoUpdateInterval.value, 1), 168)
        setTimeout(() => {
          syncStatus.value = 'idle'
          syncStatusText.value = 'Ready'
        }, 3000)
        return
      }

      // Round to nearest integer
      autoUpdateInterval.value = Math.round(autoUpdateInterval.value)
    }

    try {
      settingsStore.setAutoUpdateHosts(autoUpdateEnabled.value, autoUpdateInterval.value)
      syncStatus.value = 'success'
      syncStatusText.value = 'Settings saved successfully'
      lastSyncTime.value = new Date().toLocaleString()

      setTimeout(() => {
        syncStatus.value = 'idle'
        syncStatusText.value = autoUpdateEnabled.value ? 'Auto-sync enabled' : 'Auto-sync disabled'
      }, 2000)
    } catch (error) {
      syncStatus.value = 'error'
      syncStatusText.value = `Failed to save settings: ${error}`
      setTimeout(() => {
        syncStatus.value = 'idle'
        syncStatusText.value = 'Error saving settings'
      }, 3000)
    }
  }

  const triggerManualSync = async () => {
    if (!autoUpdateEnabled.value) {
      syncStatus.value = 'error'
      syncStatusText.value = 'Please enable auto-sync first'
      setTimeout(() => {
        syncStatus.value = 'idle'
        syncStatusText.value = 'Auto-sync disabled'
      }, 2000)
      return
    }

    syncStatus.value = 'in_progress'
    syncStatusText.value = 'Starting sync...'

    try {
      await invoke('trigger_manual_sync')
      // Success is handled by the event listener
    } catch (error) {
      console.error('Manual sync error:', error)
      syncStatus.value = 'error'
      syncStatusText.value = error instanceof Error ? error.message : 'Sync failed: Unknown error'
      setTimeout(() => {
        syncStatus.value = 'idle'
        syncStatusText.value = 'Auto-sync enabled'
      }, 5000)
    }
  }

  // Initialize status based on current settings
  if (autoUpdateEnabled.value) {
    syncStatusText.value = 'Auto-sync enabled'
  } else {
    syncStatusText.value = 'Auto-sync disabled'
  }

  // Listen for sync status updates
  listen<{ status: string; message?: string }>('sync-status-update', (event) => {
    const { status, message } = event.payload
    syncStatus.value = status as 'idle' | 'in_progress' | 'success' | 'error'
    syncStatusText.value = message || status
    if (status === 'success' || status === 'error') {
      lastSyncTime.value = new Date().toLocaleString()
    }
  })
</script>
