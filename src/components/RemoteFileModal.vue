<template>
  <transition name="fade">
    <div
      v-if="show"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 backdrop-blur-sm"
      data-tauri-drag-region
      @click.self="$emit('close')"
    >
      <transition name="slide-up">
        <div v-if="show" class="w-full max-w-md p-6 bg-white rounded-lg shadow-xl dark:bg-zinc-800">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Create Remote File</h2>
          <div class="mt-4">
            <label
              for="filename"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300"
            >
              Filename
            </label>
            <input
              ref="filenameInput"
              v-model="fileName"
              type="text"
              class="block w-full px-3 py-2 mt-1 text-gray-900 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-white focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
              placeholder="Enter filename"
              @keyup.enter="create"
            >
          </div>
          <div class="mt-4">
            <label for="url" class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Remote URL
            </label>
            <input
              ref="urlInput"
              v-model="remoteUrl"
              type="url"
              class="block w-full px-3 py-2 mt-1 text-gray-900 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-white focus:outline-none focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
              placeholder="https://example.com/hosts"
              @keyup.enter="create"
            >
            <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">
              Enter a URL to a remote hosts file. The file will be downloaded and stored locally.
            </p>
          </div>
          <div
            v-if="error"
            class="mt-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md"
          >
            <p class="text-sm text-red-700 dark:text-red-400 break-words">{{ error }}</p>
          </div>
          <div class="mt-6 flex justify-end space-x-3">
            <button
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm dark:bg-zinc-700 dark:border-zinc-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-zinc-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              :disabled="isLoading"
              @click="$emit('close')"
            >
              Cancel
            </button>
            <button
              class="px-4 py-2 text-sm font-medium text-white bg-primary-600 border border-transparent rounded-md shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="!fileName.trim() || !isValidUrl || isLoading"
              @click="create"
            >
              <span v-if="isLoading" class="flex items-center">
                <svg
                  class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle>
                  <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  ></path>
                </svg>
                Creating...
              </span>
              <span v-else>Create</span>
            </button>
          </div>
        </div>
      </transition>
    </div>
  </transition>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { BaseDirectory, exists, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
  import { computed, nextTick, ref, watch } from 'vue'
  import { HostsFileType, hostsStore } from '../stores/files'

  const props = defineProps<{
    show: boolean
  }>()

  const emit = defineEmits<{
    close: []
    created: [fileId: string]
  }>()

  const filenameInput = ref<HTMLInputElement | null>(null)

  const fileName = ref('')
  const remoteUrl = ref('')
  const isLoading = ref(false)
  const error = ref('')

  const isValidUrl = computed(() => {
    if (!remoteUrl.value.trim()) return false
    try {
      const url = new URL(remoteUrl.value)
      return url.protocol === 'https:'
    } catch {
      return false
    }
  })

  watch(
    () => props.show,
    (show) => {
      if (show) {
        fileName.value = 'New Remote File'
        remoteUrl.value = ''
        isLoading.value = false
        error.value = ''
        nextTick(() => {
          filenameInput.value?.focus()
        })
      }
    },
  )

  async function create() {
    if (!fileName.value.trim() || !isValidUrl.value || isLoading.value) {
      return
    }

    isLoading.value = true
    error.value = ''

    try {
      const id = crypto.randomUUID()
      const fileNameTrimmed = fileName.value.trim()
      const remoteUrlTrimmed = remoteUrl.value.trim()

      // Ensure files directory exists
      const dirExists = await exists('files', {
        baseDir: BaseDirectory.AppData,
      })

      if (!dirExists) {
        await mkdir('files', { baseDir: BaseDirectory.AppData })
      }

      // Create empty file first
      await writeTextFile(`files/${id}.hosts`, '', {
        baseDir: BaseDirectory.AppData,
      })

      // Fetch remote content
      await invoke('fetch_remote_hosts_file', { url: remoteUrlTrimmed, fileName: `${id}.hosts` })

      // Read the fetched content
      const content = await readTextFile(`files/${id}.hosts`, {
        baseDir: BaseDirectory.AppData,
      })

      // Create file object and add to store
      const file = {
        id,
        name: fileNameTrimmed,
        isActive: false,
        isSelected: false,
        type: HostsFileType.REMOTE,
        remoteUrl: remoteUrlTrimmed,
        content,
        status: 'loaded',
      }

      hostsStore.files = [...hostsStore.files, file]
      hostsStore.saveMetadata()

      emit('created', id)
      emit('close')
    } catch (err) {
      console.error('Error creating remote file:', err)

      // Provide more user-friendly error messages
      if (err instanceof Error) {
        if (err.message.includes('fetch') || err.message.includes('network')) {
          error.value = 'Failed to fetch the remote file. Please check the URL and try again.'
        } else if (err.message.includes('permission') || err.message.includes('access')) {
          error.value = 'Permission denied. Unable to save the file locally.'
        } else if (err.message.includes('timeout')) {
          error.value = 'Request timed out. The remote server may be slow or unreachable.'
        } else {
          error.value = `Error: ${err.message}`
        }
      } else {
        error.value = String(err) || 'An unknown error occurred.'
      }
    } finally {
      isLoading.value = false
    }
  }
</script>

<style scoped>
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.2s ease;
  }

  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }

  .slide-up-enter-active,
  .slide-up-leave-active {
    transition: transform 0.2s ease-out;
  }

  .slide-up-enter-from,
  .slide-up-leave-to {
    transform: translateY(20px);
  }
</style>
