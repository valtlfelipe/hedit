<template>
  <div class="w-full bg-white dark:bg-zinc-900 flex-1 flex flex-col min-h-0">
    <div class="flex flex-1 min-h-0">
      <!-- Line numbers -->
      <div class="line-numbers select-none text-gray-500 dark:text-gray-400 text-sm font-mono py-3 px-3 text-right bg-gray-50 dark:bg-zinc-800/50 overflow-hidden">
        <div class="h-full overflow-hidden">
          <div v-for="n in lineCount" :key="n" class="leading-6">
            {{ n }}
          </div>
        </div>
      </div>

      <!-- Editor area -->
      <div class="relative flex-1 min-h-0">
        <!-- Syntax highlighted preview (overlay) -->
        <div
          class="syntax-highlight absolute inset-0 font-mono text-sm py-3 px-4 pointer-events-none whitespace-pre overflow-auto"
          v-html="highlightedContent"
        ></div>

        <!-- Actual textarea (transparent text) -->
        <textarea
          v-model="modelValue"
          @scroll="syncScroll"
          ref="textarea"
          class="w-full h-full font-mono text-sm py-3 px-4 bg-transparent text-transparent caret-gray-800 dark:caret-gray-200 resize-none outline-none leading-6 overflow-auto"
          spellcheck="false"
          autofocus
        ></textarea>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue'

const modelValue = defineModel<string>({
  type: String,
  default: '',
})

const textarea = ref<HTMLTextAreaElement | null>(null)

const lineCount = computed(() => {
  return modelValue.value.split('\n').length
})

const highlightedContent = computed(() => {
  return modelValue.value
    .split('\n')
    .map((line) => highlightLine(line))
    .join('\n')
})

const highlightLine = (line: string) => {
  // Empty line
  if (!line.trim()) return line

  // Comment line
  if (line.trim().startsWith('#')) {
    return `<span class="text-green-600 dark:text-green-400">${escapeHtml(line)}</span>`
  }

  // IP address and hostname line
  const ipRegex = /^(\s*)([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}|[0-9a-fA-F:]+)(.*)/
  const match = line.match(ipRegex)

  if (match) {
    const [, whitespace, ip, rest] = match
    return `${escapeHtml(whitespace)}<span class="text-blue-600 dark:text-blue-400">${escapeHtml(ip)}</span><span class="text-gray-800 dark:text-gray-200">${escapeHtml(rest)}</span>`
  }

  return `<span class="text-gray-800 dark:text-gray-200">${escapeHtml(line)}</span>`
}

const escapeHtml = (text: string) => {
  const map = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#039;',
  }
  return text.replace(/[&<>"']/g, (m: string) => map[m as keyof typeof map])
}

const syncScroll = (event: Event) => {
  const target = event.target as HTMLTextAreaElement
  const scrollTop = target.scrollTop
  const scrollLeft = target.scrollLeft
  const highlight = target.previousElementSibling
  if (highlight) {
    highlight.scrollTop = scrollTop
    highlight.scrollLeft = scrollLeft
  }
}

onMounted(() => {
  nextTick(() => {
    if (textarea.value) {
      textarea.value?.focus()
    }
  })
})
</script>

<style scoped>
/* Ensure the textarea and highlight overlay are perfectly aligned */
.syntax-highlight,
textarea {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  line-height: 1.5rem;
  tab-size: 4;
}

/* Custom scrollbar styling */
textarea::-webkit-scrollbar,
.syntax-highlight::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

textarea::-webkit-scrollbar-track,
.syntax-highlight::-webkit-scrollbar-track {
  background-color: #f3f4f6;
  border-radius: 6px;
}

.dark textarea::-webkit-scrollbar-track,
.dark .syntax-highlight::-webkit-scrollbar-track {
  background-color: #374151;
}

textarea::-webkit-scrollbar-thumb,
.syntax-highlight::-webkit-scrollbar-thumb {
  background-color: #9ca3af;
  border-radius: 6px;
  border: 2px solid #f3f4f6;
}

.dark textarea::-webkit-scrollbar-thumb,
.dark .syntax-highlight::-webkit-scrollbar-thumb {
  background-color: #6b7280;
  border: 2px solid #374151;
}

textarea::-webkit-scrollbar-thumb:hover,
.syntax-highlight::-webkit-scrollbar-thumb:hover {
  background-color: #6b7280;
}

.dark textarea::-webkit-scrollbar-thumb:hover,
.dark .syntax-highlight::-webkit-scrollbar-thumb:hover {
  background-color: #9ca3af;
}

/* Firefox scrollbar */
textarea,
.syntax-highlight {
  scrollbar-width: auto;
  scrollbar-color: #9ca3af #f3f4f6;
}

.dark textarea,
.dark .syntax-highlight {
  scrollbar-color: #6b7280 #374151;
}

/* Ensure line numbers stay aligned */
.line-numbers {
  min-width: 3ch;
}
</style>