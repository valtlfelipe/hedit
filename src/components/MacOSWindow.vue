<template>
  <div class="bg-gray-50 dark:bg-zinc-900 rounded-lg shadow-2xl overflow-hidden border border-gray-200 dark:border-zinc-800 min-h-screen flex flex-col">
    <!-- Window Chrome -->
    <div class="bg-gray-200/80 dark:bg-zinc-800/80 border-b border-gray-300/80 dark:border-zinc-700/80 py-2 flex items-center backdrop-blur-sm" data-tauri-drag-region>
      <!-- Traffic Light Controls -->
      <!-- <div class="flex space-x-2 mr-4">
        <button class="w-3 h-3 bg-red-500 rounded-full hover:bg-red-600 flex items-center justify-center group" @click="appWindow.close()">
          <X class="w-2 h-2 text-red-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
        <button class="w-3 h-3 bg-yellow-500 rounded-full hover:bg-yellow-600 flex items-center justify-center group" @click="appWindow.minimize()">
          <Minus class="w-2 h-2 text-yellow-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
        <button class="w-3 h-3 bg-green-500 rounded-full hover:bg-green-600 flex items-center justify-center group" @click="appWindow.maximize()">
          <Square class="w-2 h-2 text-green-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
      </div> -->

      <!-- Window Title -->
      <div class="flex-1 text-center select-none cursor-default">
        <h1 class="text-sm font-semibold text-gray-800 dark:text-gray-100" data-tauri-drag-region>{{ title }}</h1>
      </div>

      <div class="px-4">
        <button class="p-1.5 hover:bg-gray-300/80 dark:hover:bg-zinc-700/80 rounded-md transition-colors" @click="showSettings = !showSettings">
          <Settings class="w-4 h-4 text-gray-600 dark:text-gray-200" />
        </button>

        <div v-show="showSettings" @blur="showSettings = false" tabindex="-1" class="absolute right-3 top-12 z-10 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-56">
          <ul class="text-sm text-gray-800 dark:text-gray-200">
            <li @click="toggleDarkMode" class="flex items-center gap-3 px-4 py-2.5 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer">
              <Sun class="w-4 h-4" v-if="settingsStore.isDarkTheme" />
              <Moon class="w-4 h-4" v-else />
              <span>{{ settingsStore.isDarkTheme ? 'Light Mode' : 'Dark Mode' }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <slot />
  </div>
</template>

<script setup lang="ts">
import { load } from '@tauri-apps/plugin-store'
import { Moon, Settings, Sun } from 'lucide-vue-next'
import { ref } from 'vue'
import { settingsStore } from '../stores/settings'

// import { getCurrentWindow } from '@tauri-apps/api/window';
// const appWindow = getCurrentWindow();

const showSettings = ref(false)

const toggleDarkMode = async () => {
  const store = await load('settings.json', { autoSave: false })
  await store.set('theme', {
    value: document.documentElement.classList.contains('dark') ? 'light' : 'dark',
  })
  await store.save()
  settingsStore.set(!settingsStore.isDarkTheme)

  showSettings.value = false
}

interface Props {
  title: string
}

defineProps<Props>()
</script>