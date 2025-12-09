<template>
  <div
    class="bg-gray-50/80 dark:bg-zinc-900/80 border-b border-gray-200 dark:border-zinc-800 px-3 py-2 flex items-center"
    @contextmenu.prevent="null"
  >
    <div class="flex items-center space-x-2 grow select-none">
      <div ref="createContainer" class="relative">
        <button
          class="flex items-center space-x-2 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 dark:text-gray-200 dark:hover:bg-zinc-700/80 rounded-md transition-colors"
          @click="showCreateDropdown = !showCreateDropdown"
        >
          <Plus class="w-4 h-4" />
          <span>Create</span>
          <ChevronDown class="w-4 h-4" />
        </button>

        <transition name="fade-scale">
          <div v-if="showCreateDropdown" class="absolute left-0 top-8 z-60 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-48">
            <ul class="p-1 text-sm text-gray-800 dark:text-gray-200">
              <li class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out" @click="$emit('createLocalFile'); showCreateDropdown = false">
                <File class="w-4 h-4" />
                <span>New Local File</span>
              </li>
              <li class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out" @click="$emit('createRemoteFile'); showCreateDropdown = false">
                <Globe class="w-4 h-4" />
                <span>New Remote File</span>
              </li>
            </ul>
          </div>
        </transition>
      </div>

      <Tooltip text="Save File" :shortcut="`${modifier}+S`">
        <button
          class="flex items-center space-x-2 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 dark:text-gray-200 dark:hover:bg-zinc-700/80 rounded-md transition-colors"
          @click="$emit('saveFile')"
        >
          <Save class="w-4 h-4 text-gray-600 dark:text-gray-300" />
          <span>Save</span>
        </button>
      </Tooltip>

      <Tooltip text="Activate File" :shortcut="`${modifier}+⇧+A`">
        <button
          class="flex items-center space-x-2 px-3 py-1.5 text-sm text-white bg-purple-600 hover:bg-purple-700 rounded-md transition-colors"
          :disabled="!allowActivate"
          :class="{ 'opacity-50 cursor-not-allowed': !allowActivate }"
          @click="$emit('activateFile')"
        >
          <Play class="w-4 h-4" />
        </button>
      </Tooltip>

      <div class="flex-1"></div>

      <!-- View Options -->
      <!-- <button class="flex items-center space-x-1 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200 rounded-md transition-colors">
        <span>View</span>
        <ChevronDown class="w-4 h-4" />
      </button> -->
    </div>
    <div class="flex items-center gap-2 select-none">
      <!-- Update Available Icon -->
      <Tooltip v-if="updateAvailable" text="Update Available" position="right">
        <button
          class="p-1.5 hover:bg-gray-300/80 dark:hover:bg-zinc-700/80 rounded-md transition-colors relative"
          @click="openUpdatePage"
        >
          <Download class="w-4 h-4 text-gray-600 dark:text-gray-200" />
          <span class="absolute -top-1 -right-1 w-2 h-2 bg-purple-600 rounded-full"></span>
        </button>
      </Tooltip>

      <div ref="settingsContainer" class="relative">
        <Tooltip text="Settings" position="right">
          <button class="p-1.5 hover:bg-gray-300/80 dark:hover:bg-zinc-700/80 rounded-md transition-colors" @click="showSettings = !showSettings">
            <Settings class="w-4 h-4 text-gray-600 dark:text-gray-200" />
          </button>
        </Tooltip>

        <transition name="fade-scale">
          <div v-if="showSettings" class="absolute right-0 top-8 z-60 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-50">
            <ul class="p-1 text-sm text-gray-800 dark:text-gray-200">
              <li class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out" @click="toggleDarkMode">
                <Sun v-if="settingsStore.isDarkTheme" class="w-4 h-4" />
                <Moon v-else class="w-4 h-4" />
                <span>{{ settingsStore.isDarkTheme ? 'Light Mode' : 'Dark Mode' }}</span>
              </li>
              <li class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out" @click="$emit('showLicenseModal')">
                <KeyRound class="w-4 h-4" />
                <span>Activate License</span>
              </li>
              <div class="border-t border-gray-200 dark:border-zinc-700 my-1"></div>
              <li class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out" @click="openFeedbackLink">
                <MessageSquare class="w-4 h-4" />
                <span>Feedback</span>
              </li>
            </ul>
          </div>
        </transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Tooltip from './Tooltip.vue'
import { usePlatform } from '../composables/usePlatform'
import { openUrl } from '@tauri-apps/plugin-opener'
import { ChevronDown, Download, KeyRound, MessageSquare, Moon, Play, Plus, Save, Settings, Sun, File, Globe } from 'lucide-vue-next'
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { settingsStore } from '../stores/settings'
import { listen } from '@tauri-apps/api/event'

interface UpdateInfo {
  available: boolean
  latest_version: string
  download_url: string
  current_version: string
}

const { modifier } = usePlatform()

defineProps<{
  allowActivate: boolean
}>()

defineEmits<{
  createLocalFile: []
  createRemoteFile: []
  saveFile: []
  activateFile: []
  showLicenseModal: []
}>()

const showSettings = ref(false)
const settingsContainer = ref<HTMLElement | null>(null)
const showCreateDropdown = ref(false)
const createContainer = ref<HTMLElement | null>(null)
const updateAvailable = ref(false)
const updateInfo = ref<UpdateInfo | null>(null)

const handleClickOutside = (event: MouseEvent) => {
  if (settingsContainer.value && !settingsContainer.value.contains(event.target as Node)) {
    showSettings.value = false
  }
  if (createContainer.value && !createContainer.value.contains(event.target as Node)) {
    showCreateDropdown.value = false
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    showSettings.value = false
    showCreateDropdown.value = false
  }
}

listen<UpdateInfo>('update-available', (event) => {
  updateInfo.value = event.payload
  updateAvailable.value = true
})

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

const openUpdatePage = () => {
  if (updateInfo.value?.download_url) {
    openUrl(updateInfo.value.download_url)
  }
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
