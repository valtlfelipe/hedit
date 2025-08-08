<template>
  <div class="bg-white rounded-lg shadow-2xl overflow-hidden border border-gray-200 min-h-screen">
    <!-- Window Chrome -->
    <div class="bg-gray-50 border-b border-gray-200 px-4 py-2 flex items-center" data-tauri-drag-region>
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
        <h1 class="text-sm font-medium text-gray-700" data-tauri-drag-region>{{ title }}</h1>
      </div>

      <!-- Right spacer to balance traffic lights -->
      <div class="w-16"></div>
    </div>

    <slot />
  </div>
</template>

<script setup lang="ts">
// import { X, Minus, Square } from 'lucide-vue-next';
// import { getCurrentWindow } from '@tauri-apps/api/window';

// const appWindow = getCurrentWindow();

interface Props {
  title: string;
}

defineProps<Props>();
</script>