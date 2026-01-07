<template>
  <transition name="fade">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
      data-tauri-drag-region
      @click.self="close"
    >
      <div
        class="bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-full max-w-2xl mx-4"
      >
        <div class="flex flex-col h-150">
          <!-- Header -->
          <div
            class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-zinc-700"
          >
            <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">Settings</h2>
            <button
              class="p-1 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-gray-200 dark:hover:bg-zinc-700"
              @click="close"
            >
              <X class="w-5 h-5"/>
            </button>
          </div>

          <!-- Main Content -->
          <div class="flex flex-1 overflow-hidden">
            <!-- Sidebar Navigation -->
            <div class="w-48 border-r border-gray-200 dark:border-zinc-700 overflow-y-auto">
              <nav class="p-2 space-y-1">
                <button
                  v-for="tab in tabs"
                  :key="tab.id"
                  @click="activeTab = tab.id"
                  class="w-full flex items-center gap-2 px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 rounded-md transition-colors"
                  :class="{
                  'bg-primary-500/10 text-primary-700 dark:bg-primary-500/20 dark:text-primary-300': activeTab === tab.id
                }"
                >
                  <component :is="tab.icon" class="w-4 h-4"/>
                  <span>{{ tab.name }}</span>
                </button>
              </nav>
            </div>

            <!-- Tab Content -->
            <div class="flex-1 overflow-y-auto p-4">
              <!-- General Settings -->
              <GeneralSettingsTab v-if="activeTab === 'general'"/>

              <!-- Auto Sync Settings -->
              <AutoSyncSettingsTab v-if="activeTab === 'auto-sync'"/>

              <!-- License Settings -->
              <LicenseSettingsTab v-if="activeTab === 'license'"/>

              <!-- About -->
              <AboutSettingsTab v-if="activeTab === 'about'"/>
            </div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { ref, watch, onMounted, onUnmounted } from 'vue'
  import { X, Info, Settings as SettingsIcon, KeyRound, RefreshCw } from 'lucide-vue-next'
  import GeneralSettingsTab from './settings/GeneralSettingsTab.vue'
  import AutoSyncSettingsTab from './settings/AutoSyncSettingsTab.vue'
  import LicenseSettingsTab from './settings/LicenseSettingsTab.vue'
  import AboutSettingsTab from './settings/AboutSettingsTab.vue'

  const props = defineProps<{
    show: boolean
    initialTab?: string
  }>()

  const emit = defineEmits<{
    close: []
  }>()

  const tabs = [
    { id: 'general', name: 'General', icon: SettingsIcon },
    { id: 'auto-sync', name: 'Auto Sync', icon: RefreshCw },
    { id: 'license', name: 'License', icon: KeyRound },
    { id: 'about', name: 'About', icon: Info },
  ]

  const activeTab = ref(props.initialTab || 'general')

  // Reset to initial tab when modal opens
  watch(
    () => props.show,
    (newShow) => {
      if (newShow) {
        activeTab.value = props.initialTab || 'general'
      }
    },
  )

  const close = () => {
    emit('close')
  }

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      close()
    }
  }

  onMounted(() => {
    window.addEventListener('keydown', handleKeydown)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
  })
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
