<template>
  <MacOSWindow title="Hedit">
    <Toolbar @create-file="handleCreateFile" @remove-file="handleRemoveFile" @save-file="handleSaveFile"
      @activate-file="handleActivateFile" />

    <div class="flex h-full">
      <Sidebar :files="files" @file-select="handleFileSelect" :status="selectedFile?.status || ''" />

      <CodeEditor :content="selectedFile?.content || ''" @change="handleContentChange" />
    </div>
  </MacOSWindow>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue';
import MacOSWindow from './components/MacOSWindow.vue';
import Toolbar from './components/Toolbar.vue';
import Sidebar from './components/Sidebar.vue';
import CodeEditor from './components/CodeEditor.vue';
import type { FileItem } from './components/Sidebar.vue';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';

async function readHostsFile() {
  try {
    const content = await readTextFile('/etc/hosts');

    files.value.push({
      id: '1',
      name: 'Original File',
      isActive: true,
      isSelected: true,
      content: content as string,
      status: 'Loaded successfully'
    });

  } catch (error) {
    console.error(error);
  }
}

async function resetStatus() {
  if (!selectedFile.value) {
    return;
  }

  setTimeout(() => {
    if (!selectedFile.value) {
      return;
    }

    selectedFile.value.status = '';
  }, 3000);

}

async function writeHostsFile() {
  if (!selectedFile.value) {
    return;
  }

  selectedFile.value.status = 'saving';

  try {
    await writeTextFile('/etc/hosts', selectedFile.value.content);
    selectedFile.value.status = 'saved';
    resetStatus()
    console.log("Hosts file saved successfully");
  } catch (error) {
    selectedFile.value.status = 'error';
    console.error(error);
  }
}

onMounted(() => {
  readHostsFile();
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});

function handleKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault();
    handleSaveFile();
  }
}

interface FileItemWithContent extends FileItem {
  content: string;
  status: string;
}

const files = ref<FileItemWithContent[]>([]);

const selectedFileId = ref('1');

const selectedFile = computed(() =>
  files.value.find(f => f.id === selectedFileId.value)
);

const handleFileSelect = (fileId: string) => {
  selectedFileId.value = fileId;
  files.value = files.value.map(f => ({
    ...f,
    isSelected: f.id === fileId
  }));
};

const handleContentChange = (content: string) => {
  files.value = files.value.map(f =>
    f.id === selectedFileId.value
      ? { ...f, content, status: 'modified' }
      : f
  );
};

const handleCreateFile = () => {
  const newFile: FileItemWithContent = {
    id: Date.now().toString(),
    name: `New File ${files.value.length}`,
    isActive: false,
    isSelected: false,
    content: '',
    status: ''
  };
  files.value = [...files.value, newFile];
};

const handleRemoveFile = () => {
  if (files.value.length > 1) {
    const updatedFiles = files.value.filter(f => f.id !== selectedFileId.value);
    files.value = updatedFiles;
    selectedFileId.value = updatedFiles[0]?.id || '';
  }
};

const handleSaveFile = () => {
  console.log('Saving file:', selectedFile.value?.name);
  writeHostsFile();
};

const handleActivateFile = () => {
  files.value = files.value.map(f => ({
    ...f,
    isActive: f.id === selectedFileId.value
  }));
};
</script>