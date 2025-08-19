<template>
  <div
    class="bg-gray-50 dark:bg-zinc-900 rounded-lg shadow-2xl overflow-hidden border border-gray-200 dark:border-zinc-800 h-screen flex flex-col"
    :class="{ 'macos-window': isMacOS }"
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

    <!-- Linux Window Controls -->
    <div
      v-else
      class="bg-gray-200/80 dark:bg-zinc-800/80 border-b border-gray-300/80 dark:border-zinc-700/80 py-2 px-4 flex items-center justify-between backdrop-blur-sm"
    >
      <!-- Window Controls (Linux) -->
      <div class="flex space-x-2">
        <button
          class="w-6 h-6 bg-red-500 rounded-full hover:bg-red-600 flex items-center justify-center group"
          @click="appWindow.close()"
        >
          <X class="w-3 h-3 text-red-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
        <button
          class="w-6 h-6 bg-yellow-500 rounded-full hover:bg-yellow-600 flex items-center justify-center group"
          @click="appWindow.minimize()"
        >
          <Minus class="w-3 h-3 text-yellow-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
        <button
          class="w-6 h-6 bg-green-500 rounded-full hover:bg-green-600 flex items-center justify-center group"
          @click="appWindow.toggleMaximize()"
        >
          <Square class="w-3 h-3 text-green-800 opacity-0 group-hover:opacity-100 transition-opacity" />
        </button>
      </div>

      <!-- Window Title -->
      <div class="flex-1 text-center select-none cursor-default">
        <h1 class="text-sm font-semibold text-gray-800 dark:text-gray-100">{{ title }}</h1>
      </div>

      <!-- Spacer for alignment -->
      <div class="w-18"></div>
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

<style scoped>
.macos-window {
  /* Add macOS-specific styling if needed */
}

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