<template>
  <div class="flex items-center gap-2">
    <div class="relative">
      <input
        :id="id"
        v-model="computedValue"
        type="checkbox"
        class="sr-only"
        @change="handleChange"
      >
      <div
        class="w-10 h-6 bg-gray-300 rounded-full shadow-inner transition-colors duration-200 dark:bg-gray-600 cursor-pointer"
        :class="{ 'bg-primary-600': computedValue }"
        @click="toggleValue"
      ></div>
      <div
        class="absolute top-0 left-0 w-6 h-6 bg-white rounded-full shadow transition-transform duration-200"
        :class="{ 'translate-x-4': computedValue }"
        @click="toggleValue"
      ></div>
    </div>
    <label v-if="label" :for="id" class="text-sm text-gray-800 dark:text-gray-200 cursor-pointer">
      {{ label }}
    </label>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'

  const props = defineProps({
    id: {
      type: String,
      required: true,
    },
    modelValue: {
      type: Boolean,
      required: true,
    },
    label: {
      type: String,
      default: '',
    },
  })

  const emit = defineEmits(['update:modelValue', 'change'])

  const computedValue = computed({
    get: () => props.modelValue,
    set: (value) => {
      emit('update:modelValue', value)
      emit('change', value)
    },
  })

  const handleChange = (event: Event) => {
    const target = event.target as HTMLInputElement
    computedValue.value = target.checked
  }

  const toggleValue = () => {
    computedValue.value = !props.modelValue
  }
</script>
