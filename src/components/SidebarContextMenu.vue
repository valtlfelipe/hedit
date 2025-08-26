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
          @click.prevent="$emit('create')"
        >
          <Plus class="w-4 h-4 " />
          <span>Create File</span>
        </li>
      </ul>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { Plus } from 'lucide-vue-next'

defineProps<{
  x: number
  y: number
}>()


defineEmits<{
  create: []
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
