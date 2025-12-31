<template>
  <transition name="fade-scale">
    <div
      v-if="x !== null && y !== null"
      class="absolute z-10 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-40"
      :style="{ top: `${y}px`, left: `${x}px` }"
    >
      <ul class="p-1 text-sm text-gray-800 dark:text-gray-200">
        <li
          class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out select-none"
          :class="{ 'cursor-not-allowed opacity-50': isFileActive }"
          @click.prevent="isFileActive ? null : $emit('activate')"
        >
          <Play class="w-4 h-4 "/>
          <span>Activate File</span>
        </li>
        <li
          class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out select-none"
          @click.prevent="$emit('edit')"
        >
          <Pencil class="w-4 h-4"/>
          <span>Rename File</span>
        </li>
        <li
          class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out select-none"
          @click.prevent="$emit('copyId')"
        >
          <Clipboard class="w-4 h-4"/>
          <span>Copy ID</span>
        </li>
        <li
          v-if="isRemote"
          class="rounded-lg flex items-center gap-2 px-2 py-1 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out select-none"
          @click.prevent="$emit('refresh')"
        >
          <RefreshCw :class="['w-4 h-4', { 'animate-spin': isRefreshing }]"/>
          <span>Refresh</span>
        </li>
        <div class="border-t border-gray-200 dark:border-zinc-700 my-1"></div>
        <li
          class="rounded-lg flex items-center gap-2 px-2 py-1 text-red-600 dark:text-red-400 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out select-none"
          :class="{ 'cursor-not-allowed opacity-50': isFileActive }"
          @click.prevent="isFileActive ? null : $emit('delete')"
        >
          <Trash2 class="w-4 h-4"/>
          <span>Delete File</span>
        </li>
      </ul>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { Clipboard, Pencil, Play, RefreshCw, Trash2 } from 'lucide-vue-next'

  defineProps<{
    x: number
    y: number
    isFileActive: boolean
    isRemote: boolean
    isRefreshing: boolean
  }>()

  defineEmits<{
    activate: []
    edit: []
    delete: []
    refresh: []
    copyId: []
  }>()
</script>

<style scoped>
  .fade-scale-enter-active,
  .fade-scale-leave-active {
    transition:
      transform 0.1s ease,
      opacity 0.1s ease;
  }

  .fade-scale-enter-from,
  .fade-scale-leave-to {
    opacity: 0;
    transform: scale(0.95) translateY(-10px);
  }
</style>
