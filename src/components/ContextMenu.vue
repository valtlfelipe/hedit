<template>
  <transition name="fade-scale">
    <div
      class="absolute z-10 bg-gray-50/95 dark:bg-zinc-800/95 backdrop-blur-xl border border-gray-200 dark:border-zinc-700 rounded-lg shadow-lg w-56"
      :style="{ top: `${y}px`, left: `${x}px` }"
    >
      <ul class="p-1 text-sm text-gray-800 dark:text-gray-200">
        <li
          @click.prevent="$emit('edit')"
          class="rounded-lg flex items-center gap-3 px-3 py-1.5 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out"
        >
          <Pencil class="w-4 h-4" />
          <span>Rename File</span>
        </li>
        <div class="border-t border-gray-200 dark:border-zinc-700 my-1"></div>
        <li
          @click.prevent="$emit('delete')"
          class="rounded-lg flex items-center gap-3 px-3 py-1.5 text-red-600 dark:text-red-400 hover:bg-gray-200/80 dark:hover:bg-zinc-700/80 cursor-pointer transition-colors duration-150 ease-in-out"
        >
          <Trash2 class="w-4 h-4" />
          <span>Delete File</span>
        </li>
      </ul>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { Pencil, Trash2 } from 'lucide-vue-next'

defineProps<{
  x: number
  y: number
}>()

defineEmits<{
  edit: []
  delete: []
}>()
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
