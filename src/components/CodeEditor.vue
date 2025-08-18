<template>
  <div class="w-full bg-white dark:bg-zinc-900 flex-1 flex flex-col min-h-0">
    <div class="flex flex-1 min-h-0">
      <!-- Line numbers -->
      <div
        ref="lineNumbers"
        class="line-numbers select-none text-gray-500 dark:text-gray-400 text-sm font-mono py-3 px-4 text-right bg-gray-50 dark:bg-zinc-800/50 overflow-y-auto overflow-x-hidden"
        role="presentation"
        aria-label="Line numbers"
      >
        <div
          v-for="n in lineCount"
          :key="n"
          class="leading-6 h-6"
          :class="{ 'text-red-500': errorLines.has(n) }"
        >
          {{ n }}
        </div>
      </div>

      <!-- Editor area -->
      <div class="relative flex-1 min-h-0">
        <!-- Syntax highlighted preview (overlay) -->
        <div
          ref="highlightOverlay"
          class="syntax-highlight absolute inset-0 font-mono text-sm py-3 px-4 pointer-events-none whitespace-pre overflow-auto"
          role="presentation"
          aria-hidden="true"
        >
          <template v-for="(line, index) in highlightedLines" :key="index">
            <span v-html="line"></span><br v-if="index < highlightedLines.length - 1">
          </template>
        </div>

        <!-- Actual textarea (transparent text) -->
        <textarea
          ref="textarea"
          v-model="modelValue"
          class="editor-textarea w-full h-full font-mono text-sm py-3 px-4 bg-transparent text-transparent caret-gray-800 dark:caret-gray-200 resize-none outline-none leading-6 overflow-auto"
          :class="textareaClasses"
          spellcheck="false"
          autocomplete="off"
          autocorrect="off"
          autocapitalize="off"
          role="textbox"
          aria-label="Code editor"
          aria-multiline="true"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useScroll, watchDebounced } from '@vueuse/core'
import { computed, nextTick, onMounted, type Ref, ref, shallowRef, watch } from 'vue'

// Constants
const VALIDATION_DEBOUNCE = 300
const VALIDATION_MAX_WAIT = 800
const MIN_LINE_NUMBER_WIDTH = '3ch'

// Props & Model
const modelValue = defineModel<string>({
  type: String,
  default: '',
})

// Refs
const textarea: Ref<HTMLTextAreaElement | null> = ref(null)
const highlightOverlay: Ref<HTMLDivElement | null> = ref(null)
const lineNumbers: Ref<HTMLDivElement | null> = ref(null)
const errorLines = shallowRef(new Set<number>())
const lastValidatedContent = ref('')

// Scroll synchronization
const textareaScroll = useScroll(textarea)
const overlayScroll = useScroll(highlightOverlay)
const lineNumbersScroll = useScroll(lineNumbers)

watch(
  () => modelValue.value,
  () => {
    nextTick(() => {
      textareaScroll.measure()
      overlayScroll.measure()
      lineNumbersScroll.measure()
    })
  }
)

// Prevent circular scroll updates
const isScrollSyncing = ref(false)

// Sync scroll positions
const syncScrollPositions = (x: number, y: number) => {
  if (isScrollSyncing.value) return

  isScrollSyncing.value = true

  nextTick(() => {
    // Sync overlay with both X and Y coordinates
    overlayScroll.x.value = x
    overlayScroll.y.value = y

    // Sync line numbers with Y coordinate only (vertical scroll)
    lineNumbersScroll.y.value = y

    isScrollSyncing.value = false
  })
}

// Watch for textarea scroll changes (primary scroll source)
watch(
  () => [textareaScroll.x.value, textareaScroll.y.value],
  ([x, y]) => syncScrollPositions(x, y),
  { flush: 'sync' },
)

// Watch for line numbers scroll changes (when user scrolls line numbers directly)
watch(
  () => lineNumbersScroll.y.value,
  (y) => {
    if (!isScrollSyncing.value) {
      isScrollSyncing.value = true
      nextTick(() => {
        // Sync textarea and overlay to match line numbers scroll
        textareaScroll.y.value = y
        overlayScroll.y.value = y
        isScrollSyncing.value = false
      })
    }
  },
  { flush: 'sync' },
)

// Computed Properties
const lineCount = computed((): number => {
  const content = modelValue.value
  return content ? content.split('\n').length : 1
})

const textareaClasses = computed(() => ({
  'has-errors': errorLines.value.size > 0,
}))

const lines = computed((): string[] => {
  return modelValue.value.split('\n')
})

const highlightedLines = computed((): string[] => {
  return lines.value.map((line, index) => highlightLine(line, index + 1))
})

// IP Validation - More robust regex patterns
const IPV4_PATTERN =
  /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/
const IPV6_PATTERN = /^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$|^::1$|^::$/

// Validation Functions
const isValidIpAddress = (ip: string): boolean => {
  return IPV4_PATTERN.test(ip) || IPV6_PATTERN.test(ip)
}

const validateLine = (line: string): boolean => {
  const trimmedLine = line.trim()

  // Empty lines and comments are valid
  if (trimmedLine === '' || trimmedLine.startsWith('#')) {
    return true
  }

  const parts = trimmedLine.split(/\s+/)
  if (parts.length < 2) {
    return false
  }

  const ip = parts[0]
  return isValidIpAddress(ip)
}

const validateContent = (content: string): Set<number> => {
  const newErrorLines = new Set<number>()
  const contentLines = content.split('\n')

  contentLines.forEach((line, index) => {
    if (!validateLine(line)) {
      newErrorLines.add(index + 1)
    }
  })

  return newErrorLines
}

// HTML Escaping - Fixed regex and more comprehensive
const HTML_ESCAPE_MAP: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#x27;',
  '/': '&#x2F;',
} as const

const escapeHtml = (text: string): string => {
  return text.replace(/[&<>"'/]/g, (char) => HTML_ESCAPE_MAP[char] || char)
}

// Syntax Highlighting
const highlightLine = (line: string, lineNumber: number): string => {
  const escapedLine = escapeHtml(line)

  if (errorLines.value.has(lineNumber)) {
    return `<span class="text-gray-800 dark:text-gray-200 invalid-line">${escapedLine}</span>`
  }

  // Empty line
  if (!line.trim()) {
    return escapedLine
  }

  // Comment line
  if (line.trim().startsWith('#')) {
    return `<span class="text-green-600 dark:text-green-400">${escapedLine}</span>`
  }

  // IP address and hostname line
  const ipMatch = line.match(/^(\s*)([^\s]+)(.*)/)

  if (ipMatch) {
    const [, whitespace, ip, rest] = ipMatch
    const escapedWhitespace = escapeHtml(whitespace)
    const escapedIp = escapeHtml(ip)
    const escapedRest = escapeHtml(rest)

    if (isValidIpAddress(ip)) {
      return `${escapedWhitespace}<span class="text-ip">${escapedIp}</span><span class="text-gray-800 dark:text-gray-200">${escapedRest}</span>`
    }
  }

  return `<span class="text-gray-800 dark:text-gray-200">${escapedLine}</span>`
}

// Validation Watcher - Only revalidate if content actually changed
watchDebounced(
  modelValue,
  (newValue: string) => {
    if (newValue !== lastValidatedContent.value) {
      errorLines.value = validateContent(newValue)
      lastValidatedContent.value = newValue
    }
  },
  {
    immediate: true,
    debounce: VALIDATION_DEBOUNCE,
    maxWait: VALIDATION_MAX_WAIT,
  },
)

const focus = (): void => {
  nextTick(() => {
    textarea.value?.focus()
  })
}

defineExpose({
  focus,
  hasErrors: computed(() => errorLines.value.size > 0),
})

onMounted(() => {
  focus()
})
</script>

<style scoped>
/* Ensure the textarea and highlight overlay are perfectly aligned */
.syntax-highlight,
.editor-textarea {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  line-height: 1.5rem;
  tab-size: 4;
  white-space: pre;
}

/* Ensure line numbers have identical vertical spacing */
.line-numbers {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.875rem;
  line-height: 1.5rem;
  box-sizing: border-box;
}

/* Force consistent line height for line numbers */
.line-numbers div {
  height: 1.5rem;
  line-height: 1.5rem;
}

.invalid-line {
  text-decoration: underline;
  text-decoration-color: #ef4444;
  text-decoration-style: wavy;
  text-underline-offset: 0.2em;
}

.text-ip {
  color: #3b82f6;
}

.dark .text-ip {
  color: #60a5fa;
}

.editor-textarea:focus {
  outline: none;
}

/* Custom scrollbar styling */
.editor-textarea::-webkit-scrollbar,
.syntax-highlight::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

.editor-textarea::-webkit-scrollbar-track,
.syntax-highlight::-webkit-scrollbar-track {
  background-color: #f3f4f6;
  border-radius: 6px;
}

.dark .editor-textarea::-webkit-scrollbar-track,
.dark .syntax-highlight::-webkit-scrollbar-track {
  background-color: #374151;
}

.editor-textarea::-webkit-scrollbar-thumb,
.syntax-highlight::-webkit-scrollbar-thumb {
  background-color: #9ca3af;
  border-radius: 6px;
  border: 2px solid #f3f4f6;
}

.dark .editor-textarea::-webkit-scrollbar-thumb,
.dark .syntax-highlight::-webkit-scrollbar-thumb {
  background-color: #6b7280;
  border: 2px solid #374151;
}

.editor-textarea::-webkit-scrollbar-thumb:hover,
.syntax-highlight::-webkit-scrollbar-thumb:hover {
  background-color: #6b7280;
}

.dark .editor-textarea::-webkit-scrollbar-thumb:hover,
.dark .syntax-highlight::-webkit-scrollbar-thumb:hover {
  background-color: #9ca3af;
}

/* Firefox scrollbar */
.editor-textarea,
.syntax-highlight {
  scrollbar-width: auto;
  scrollbar-color: #9ca3af #f3f6;
}

.dark .editor-textarea,
.dark .syntax-highlight {
  scrollbar-color: #6b7280 #374151;
}

/* Hide scrollbars for line numbers while keeping scroll functionality */
.line-numbers {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* Internet Explorer 10+ */
}

.line-numbers::-webkit-scrollbar {
  display: none; /* WebKit */
}

/* Ensure line numbers stay aligned */
.line-numbers {
  min-width: v-bind(MIN_LINE_NUMBER_WIDTH);
}

/* Performance optimization for large files */
.syntax-highlight {
  contain: layout style paint;
}

.editor-textarea {
  contain: layout style;
}

/* Ensure perfect pixel alignment */
.line-numbers,
.syntax-highlight,
.editor-textarea {
  box-sizing: border-box;
}
</style>