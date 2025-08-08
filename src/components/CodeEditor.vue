<template>
  <div class="w-full bg-white min-h-screen">
    <div class="flex overflow-hidden min-h-full">
      <!-- Line numbers -->
      <div class="line-numbers select-none text-gray-500 text-sm font-mono py-3 px-3 text-right bg-gray-50">
        <div v-for="n in lineCount" :key="n" class="leading-5">
          {{ n }}
        </div>
      </div>

      <!-- Editor area -->
      <div class="relative flex-1 h-full w-full">
        <!-- Syntax highlighted preview (overlay) -->
        <div
          class="syntax-highlight absolute inset-0 font-mono text-sm py-3 px-4 pointer-events-none whitespace-pre overflow-auto"
          v-html="highlightedContent"
        ></div>

        <!-- Actual textarea (transparent text) -->
        <textarea
          :value="content"
          @input="updateContent"
          @scroll="syncScroll"
          ref="textarea"
          class="w-full min-h-full font-mono text-sm py-3 px-4 bg-transparent text-transparent caret-gray-800 resize-none outline-none leading-6"
          spellcheck="false"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface Props {
  content: string;
}

const props = defineProps<Props>()

const emit = defineEmits<{
  change: [content: string];
}>();

const textarea = ref<HTMLTextAreaElement | null>(null)

const lineCount = computed(() => {
  return props.content.split('\n').length
})

const highlightedContent = computed(() => {
  return props.content
    .split('\n')
    .map(line => highlightLine(line))
    .join('\n')
})

const highlightLine = (line: string) => {
  // Empty line
  if (!line.trim()) return line

  // Comment line
  if (line.trim().startsWith('#')) {
    return `<span class="text-green-600">${escapeHtml(line)}</span>`
  }

  // IP address and hostname line
  const ipRegex = /^(\s*)([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}|[0-9a-fA-F:]+)(.*)/
  const match = line.match(ipRegex)

  if (match) {
    const [, whitespace, ip, rest] = match
    return `${escapeHtml(whitespace)}<span class="text-blue-600">${escapeHtml(ip)}</span><span class="text-gray-800">${escapeHtml(rest)}</span>`
  }

  return `<span class="text-gray-800">${escapeHtml(line)}</span>`
}

const escapeHtml = (text: string) => {
  const map = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#039;'
  }
  return text.replace(/[&<>"']/g, (m: string) => map[m as keyof typeof map])
}

const updateContent = (e: Event) => {
  const target = e.target as HTMLTextAreaElement;
  emit('change', target.value);
}

const syncScroll = (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  const scrollTop = target.scrollTop
  const scrollLeft = target.scrollLeft
  const highlight = target.previousElementSibling
  if (highlight) {
    highlight.scrollTop = scrollTop
    highlight.scrollLeft = scrollLeft
  }
}

onMounted(() => {
  // Set initial height if needed
  if (textarea.value) {
    textarea.value.style.height = '400px'
  }
})
</script>

<style scoped>
/* Ensure the textarea and highlight overlay are perfectly aligned */
.syntax-highlight,
textarea {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  line-height: 1.3rem;
  tab-size: 4;
}

/* Custom scrollbar for dark theme */
textarea::-webkit-scrollbar,
.syntax-highlight::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

textarea::-webkit-scrollbar-track,
.syntax-highlight::-webkit-scrollbar-track {
  background-color: #1F2937;
}

textarea::-webkit-scrollbar-thumb,
.syntax-highlight::-webkit-scrollbar-thumb {
  background-color: #4B5563;
}

textarea::-webkit-scrollbar-thumb:hover,
.syntax-highlight::-webkit-scrollbar-thumb:hover {
  background-color: #6B7280;
}

/* Firefox scrollbar */
textarea,
.syntax-highlight {
  scrollbar-width: thin;
  scrollbar-color: #4B5563 #1F2937;
}

/* Ensure line numbers stay aligned */
.line-numbers {
  min-width: 3ch;
}
</style>