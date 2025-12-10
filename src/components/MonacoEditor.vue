<template>
  <div class="h-full">
    <div v-show="isReady" ref="editorContainer" class="w-full h-full"></div>
    <LoadingSpinner v-if="!isReady" class="m-auto"/>
  </div>
</template>

<script setup lang="ts">
  // import * as monaco from 'monaco-editor'
  // import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'

  // @ts-expect-error
  import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'
  import 'monaco-editor/esm/vs/editor/contrib/comment/browser/comment.js'
  import 'monaco-editor/esm/vs/editor/contrib/find/browser/findController.js'
  import 'monaco-editor/esm/vs/editor/contrib/find/browser/findWidget.css'
  import 'monaco-editor/esm/vs/editor/contrib/hover/browser/hoverContribution.js'
  import 'monaco-editor/esm/vs/base/browser/ui/codicons/codicon/codicon.css'

  import { nextTick, onBeforeMount, onMounted, onUnmounted, ref, watch } from 'vue'
  import LoadingSpinner from './LoadingSpinner.vue'

  const isReady = ref(false)

  // self.MonacoEnvironment = {
  //   getWorker() {
  //     return new editorWorker()
  //   },
  // }

  // --- Constants ---
  const IPV4_REGEX =
    /^(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}$/
  const IPV6_REGEX =
    /^(?:(?:[a-fA-F\d]{1,4}:){7}(?:[a-fA-F\d]{1,4}|:)|(?:[a-fA-F\d]{1,4}:){6}(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|:[a-fA-F\d]{1,4}|:)|(?:[a-fA-F\d]{1,4}:){5}(?::(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,2}|:)|(?:[a-fA-F\d]{1,4}:){4}(?:(?::[a-fA-F\d]{1,4}){0,1}:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,3}|:)|(?:[a-fA-F\d]{1,4}:){3}(?:(?::[a-fA-F\d]{1,4}){0,2}:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,4}|:)|(?:[a-fA-F\d]{1,4}:){2}(?:(?::[a-fA-F\d]{1,4}){0,3}:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,5}|:)|(?:[a-fA-F\d]{1,4}:){1}(?:(?::[a-fA-F\d]{1,4}){0,4}:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,6}|:)|(?::(?:(?::[a-fA-F\d]{1,4}){0,5}:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}|(?::[a-fA-F\d]{1,4}){1,7}|:)))(?:%[0-9a-zA-Z]{1,})?$/i
  const HOSTNAME_REGEX =
    /^(([a-zA-Z0-9_]|[a-zA-Z0-9_][a-zA-Z0-9_-]*[a-zA-Z0-9_])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9-]*[A-Za-z0-9])$/

  // --- Props & Emits ---
  const props = defineProps<{
    modelValue: string
    isDarkTheme?: boolean
  }>()
  const emit = defineEmits(['update:modelValue', 'validation-status'])

  // --- Monaco Setup (run once) ---
  let isMonacoSetup = false
  function setupMonaco() {
    if (isMonacoSetup) return

    monaco.languages.register({ id: 'hosts' })
    monaco.languages.setMonarchTokensProvider('hosts', {
      tokenizer: {
        root: [
          [/#.*$/, 'comment'],
          [/(?:[0-9]{1,3}\.){3}[0-9]{1,3}/, 'number'], // IPv4
          [/([a-fA-F0-9:]+:+)+[a-fA-F0-9]+/, 'number'], // IPv6
          [/[a-zA-Z0-9\-. Advance]/, 'string'], // Hostnames
        ],
      },
    })
    monaco.languages.setLanguageConfiguration('hosts', {
      comments: {
        lineComment: '#',
      },
    })

    monaco.editor.defineTheme('HeditLight', {
      base: 'vs',
      inherit: true,
      rules: [
        { token: 'number', foreground: 'db70b8' },
        { token: 'string', foreground: '000000' },
      ],
      colors: {
        'editor.background': '#f9fafb',
        'editorLineNumber.foreground': '#888888',
        'editorLineNumber.activeForeground': '#000000',
      },
    })
    monaco.editor.defineTheme('HeditDark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'number', foreground: 'db70b8' },
        { token: 'string', foreground: 'ffffff' },
      ],
      colors: { 'editor.background': '#18181b' },
    })

    isMonacoSetup = true
  }

  // --- Hosts File Validation ---
  function isValidIp(ip: string): boolean {
    return IPV4_REGEX.test(ip) || IPV6_REGEX.test(ip)
  }

  function isValidHostname(hostname: string): boolean {
    return HOSTNAME_REGEX.test(hostname)
  }

  function getHostsValidationMarkers(content: string): monaco.editor.IMarkerData[] {
    const markers: monaco.editor.IMarkerData[] = []
    const lines = content.split('\n')

    lines.forEach((lineContent, i) => {
      const line = lineContent.trim()
      const lineNumber = i + 1

      if (line.startsWith('#') || line === '') {
        return
      }

      const parts = line.split(/\s+/)
      const ip = parts[0]
      const hostnames = parts.slice(1)

      if (!isValidIp(ip)) {
        markers.push({
          message: `Invalid IP address: ${ip}`,
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: lineNumber,
          startColumn: lineContent.indexOf(ip) + 1,
          endLineNumber: lineNumber,
          endColumn: lineContent.indexOf(ip) + ip.length + 1,
        })
      }

      if (hostnames.length === 0) {
        markers.push({
          message: 'At least one hostname is required.',
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: lineNumber,
          startColumn: lineContent.indexOf(ip) + ip.length + 2,
          endLineNumber: lineNumber,
          endColumn: lineContent.length + 1,
        })
      } else {
        let initiatedComments = false
        hostnames.forEach((hostname) => {
          if (hostname.startsWith('#') || initiatedComments) {
            initiatedComments = true
            return
          }

          if (!isValidHostname(hostname)) {
            const startColumn = lineContent.indexOf(hostname) + 1
            markers.push({
              message: `Invalid hostname: ${hostname}`,
              severity: monaco.MarkerSeverity.Error,
              startLineNumber: lineNumber,
              startColumn,
              endLineNumber: lineNumber,
              endColumn: startColumn + hostname.length,
            })
          }
        })
      }
    })
    return markers
  }

  function validate(editorInstance: monaco.editor.IStandaloneCodeEditor) {
    const model = editorInstance.getModel()
    if (!model) return

    const content = editorInstance.getValue()
    const markers = getHostsValidationMarkers(content)

    monaco.editor.setModelMarkers(model, 'hedit-validator', markers)
    emit('validation-status', markers.length === 0)
  }

  // --- Component Logic ---
  const editorContainer = ref<HTMLElement | null>(null)
  let editor: monaco.editor.IStandaloneCodeEditor | null = null
  let resizeObserver: ResizeObserver | null = null

  onBeforeMount(() => {
    setupMonaco()
  })

  onMounted(() => {
    if (!editorContainer.value) return

    editor = monaco.editor.create(editorContainer.value, {
      value: props.modelValue,
      language: 'hosts',
      theme: props.isDarkTheme ? 'HeditDark' : 'HeditLight',
      fontSize: 14,
      lineHeight: 25,
      letterSpacing: 0.2,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      renderLineHighlight: 'none',
      glyphMargin: false,
      showUnused: false,
      showDeprecated: false,
    })

    resizeObserver = new ResizeObserver(() => editor?.layout())
    resizeObserver.observe(editorContainer.value)

    editor.onDidChangeModelContent(() => {
      if (editor) {
        emit('update:modelValue', editor.getValue())
        validate(editor)
      }
    })

    validate(editor) // Initial validation

    isReady.value = true

    nextTick(() => {
      editor?.focus()
    })
  })

  onUnmounted(() => {
    resizeObserver?.disconnect()
    editor?.dispose()
  })

  // --- Watchers ---
  watch(
    () => props.modelValue,
    (newValue) => {
      if (editor && editor.getValue() !== newValue) {
        editor.setValue(newValue)
        editor.focus()
      }
    },
  )

  watch(
    () => props.isDarkTheme,
    (isDark) => {
      monaco.editor.setTheme(isDark ? 'HeditDark' : 'HeditLight')
      editor?.focus()
    },
  )

  defineExpose({
    focus: () => editor?.focus(),
  })
</script>
