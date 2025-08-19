<template>
  <div
    class="bg-gray-50 dark:bg-zinc-900 shadow-2xl overflow-hidden border border-gray-200 dark:border-zinc-800 h-screen flex flex-col"
    :class="{ 'rounded-lg': isMacOS }"
    data-tauri-drag-region
  >
    <!-- Window Chrome -->
    <div
      v-if="isMacOS"
      class="bg-gray-200/80 dark:bg-zinc-800/80 border-b border-gray-300/80 dark:border-zinc-700/80 py-2 flex items-center backdrop-blur-sm"
      data-tauri-drag-region
    >
      <!-- Window Title -->
      <div class="flex-1 text-center select-none cursor-default">
        <h1 class="text-sm font-semibold text-gray-800 dark:text-gray-100" data-tauri-drag-region>{{ title }}</h1>
      </div>
    </div>

    <!-- Windows/Linux Window Controls -->
    <div
      v-else
      class="bg-gray-200/80 dark:bg-zinc-800/80 border-b border-gray-300/80 dark:border-zinc-700/80 py-2 px-4 flex items-center justify-between backdrop-blur-sm"
      data-tauri-drag-region
    >
      <!-- Window Title -->
      <div class="flex-1 text-center select-none cursor-default">
        <h1 class="text-sm font-semibold text-gray-800 dark:text-gray-100" data-tauri-drag-region>{{ title }}</h1>
      </div>

      <!-- Window Controls (Windows/Linux) -->
      <div class="flex space-x-2">
        <button
          class="w-8 h-8 bg-transparent hover:bg-gray-300 dark:hover:bg-zinc-700 rounded flex items-center justify-center group"
          @click="appWindow.minimize()"
        >
          <Minus class="w-4 h-4 text-gray-800 dark:text-gray-200" />
        </button>
        <button
          class="w-8 h-8 bg-transparent hover:bg-gray-300 dark:hover:bg-zinc-700 rounded flex items-center justify-center group"
          @click="appWindow.toggleMaximize()"
        >
          <Square class="w-4 h-4 text-gray-800 dark:text-gray-200" />
        </button>
        <button
          class="w-8 h-8 bg-transparent hover:bg-red-500 rounded flex items-center justify-center group"
          @click="appWindow.close()"
        >
          <X class="w-4 h-4 text-gray-800 dark:text-gray-200 group-hover:text-white" />
        </button>
      </div>
    </div>

    <slot />
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { platform } from '@tauri-apps/plugin-os';
import { Minus, Square, X } from 'lucide-vue-next'

interface Props {
  title: string
}

defineProps<Props>()

const appWindow = getCurrentWindow()
const isMacOS = platform() === 'macos'
</script>