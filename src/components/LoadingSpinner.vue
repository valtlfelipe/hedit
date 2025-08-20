<template>
    <div class="flex items-center justify-center h-full w-full">
        <LoaderCircle class="animate-spin text-gray-400 w-10 h-10" />
    </div>
</template>

<script setup lang="ts">
import { LoaderCircle } from 'lucide-vue-next';

// No script needed for static spinner
</script>

<style scoped>
.lucide-spin {
    animation: lucide-spin 1s linear infinite;
    color: #9ca3af; /* Tailwind gray-400 */
}

@keyframes lucide-spin {
    100% {
        transform: rotate(360deg);
    }
}
</style>