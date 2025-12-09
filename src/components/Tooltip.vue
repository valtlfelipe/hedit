<template>
  <div class="relative flex items-center group">
    <slot/>
    <div
      class="absolute top-full mt-2 w-max bg-white text-gray-700 border border-gray-200 text-sm rounded-md py-1.5 px-3 opacity-0 group-hover:opacity-100 transition-opacity duration-200 delay-300 pointer-events-none dark:bg-zinc-800 dark:text-gray-200 dark:border-transparent shadow-md z-50"
      :class="{
        'left-0': position === 'left',
        'right-0': position === 'right',
      }"
    >
      <span>{{ text }}</span>
      <span v-if="shortcut" class="ml-2 text-gray-400 dark:text-gray-500">{{ shortcut }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
  withDefaults(
    defineProps<{
      text: string
      shortcut?: string | null
      position?: 'left' | 'right'
    }>(),
    {
      shortcut: null,
      position: 'left',
    },
  )
</script>
