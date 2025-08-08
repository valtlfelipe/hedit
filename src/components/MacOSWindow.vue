<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl overflow-hidden border border-gray-200 dark:border-gray-800 min-h-screen">
    <!-- Window Chrome -->
    <div class="bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600 py-2 flex items-center" data-tauri-drag-region>
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
        <h1 class="text-sm font-medium text-gray-700 dark:text-gray-200" data-tauri-drag-region>{{ title }}</h1>
      </div>

      <div class="flex space-x-2">
        <button class="p-2 hover:bg-gray-200 dark:hover:bg-gray-600 rounded transition-colors" @click="showSettings = !showSettings">
          <Settings class="w-4 h-4 text-gray-600 dark:text-gray-200" />
        </button>

        <div v-show="showSettings" class="absolute right-5 top-11 z-10 bg-white dark:bg-gray-800 divide-y divide-gray-100 dark:divide-gray-600 rounded-lg shadow-sm w-55">
          <ul class="py-2 text-sm text-gray-700 dark:text-gray-400">
            <li class="flex items-center px-4 hover:bg-gray-100 cursor-pointer dark:hover:bg-gray-600">
              <Sun class="inline w-4 h-4" v-if="settingsStore.isDarkTheme" />
              <Moon class="inline w-4 h-4" v-else />
              <a href="#" @click.prevent="toggleDarkMode" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600">
                {{ settingsStore.isDarkTheme ? 'Disable' : 'Enable' }} Dark Mode
              </a>
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