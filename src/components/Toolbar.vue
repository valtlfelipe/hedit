<template>
  <div class="bg-gray-50/80 dark:bg-zinc-900/80 border-b border-gray-200 dark:border-zinc-800 px-3 py-2 flex items-center">
    <div class="flex items-center space-x-2 flex-grow select-none">
      <button
        @click="$emit('createFile')"
        class="flex items-center space-x-2 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 dark:text-gray-200 dark:hover:bg-zinc-700/80 rounded-md transition-colors"
      >
        <Plus class="w-4 h-4" />
        <span>Create</span>
      </button>

      <button
        @click="$emit('saveFile')"
        class="flex items-center space-x-2 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 dark:text-gray-200 dark:hover:bg-zinc-700/80 rounded-md transition-colors"
      >
        <Save class="w-4 h-4 text-gray-600 dark:text-gray-300" />
        <span class="text-gray-800 dark:text-gray-200 font-medium">Save</span>
      </button>

      <button
        @click="$emit('activateFile')"
        class="flex items-center space-x-1 px-3 py-1.5 text-sm text-white bg-purple-600 hover:bg-purple-700 rounded-md transition-colors"
        :disabled="!allowActivate"
        :class="{ 'opacity-50 cursor-not-allowed': !allowActivate }"
      >
        <Play class="w-4 h-4" />
        <span>Activate</span>
      </button>

      <div class="flex-1"></div>

      <!-- View Options -->
      <!-- <button class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors">
        <span>View</span>
        <ChevronDown class="w-4 h-4" />
      </button> -->
    </div>
    <div class="relative select-none" ref="settingsContainer">
        <button class="p-1.5 hover:bg-gray-300/80 dark:hover:bg-zinc-700/80 rounded-md transition-colors" @click="showSettings = !showSettings">
          <Settings class="w-4 h-4 text-gray-600 dark:text-gray-200" />
        </button>

        <transition name="fade-scale">
          <div v-if="showSettings" class="absolute right-0 top-8 z-10 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-50">
            <ul class="p-1 text-sm text-gray-800 dark:text-gray-200">
              <li @click="toggleDarkMode" class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out">
                <Sun class="w-4 h-4" v-if="settingsStore.isDarkTheme" />
                <Moon class="w-4 h-4" v-else />
                <span>{{ settingsStore.isDarkTheme ? 'Light Mode' : 'Dark Mode' }}</span>
              </li>
              <li @click="showLicenseModal = true" class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out">
                <KeyRound class="w-4 h-4" />
                <span>Activate License</span>
              </li>
              <div class="border-t border-gray-200 dark:border-zinc-700 my-1"></div>
              <li @click="openFeedbackLink" class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out">
                <MessageSquare class="w-4 h-4" />
                <span>Feedback</span>
              </li>
            </ul>
          </div>
        </transition>
      </div>
      <LicenseModal :show="showLicenseModal" @close="showLicenseModal = false" />
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { openUrl } from '@tauri-apps/plugin-opener'
import { KeyRound, MessageSquare, Moon, Play, Plus, Save, Settings, Sun } from 'lucide-vue-next'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { settingsStore } from '../stores/settings'
import LicenseModal from './LicenseModal.vue'

defineProps<{
  allowActivate: boolean
}>()

defineEmits<{
  createFile: []
  saveFile: []
  activateFile: []
}>()

const showSettings = ref(false)
const showLicenseModal = ref(false)
const settingsContainer = ref<HTMLElement | null>(null)

listen('activate_license', async () => {
  showLicenseModal.value = true
})

const handleClickOutside = (event: MouseEvent) => {
  if (settingsContainer.value && !settingsContainer.value.contains(event.target as Node)) {
    showSettings.value = false
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    showSettings.value = false
    showLicenseModal.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('keydown', handleKeydown)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('keydown', handleKeydown)
})

const toggleDarkMode = async () => {
  settingsStore.setTheme(!settingsStore.isDarkTheme)
  showSettings.value = false
}

const openFeedbackLink = () => {
  openUrl('https://github.com/valtlfelipe/hedit/issues/new/choose')
  showSettings.value = false
}
</script>

<style scoped>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition: transform 0.1s ease, opacity 0.1s ease;
}

.fade-scale-enter-from,
.fade-scale-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-10px);
}
</style>
