export function useFileWatcher() {
  // This composable is now deprecated as we handle file watching directly in the component
  // Keeping it for now to avoid breaking changes, but it doesn't do anything

  const initializeFileWatcher = (_fileSelectedChanged: { value: boolean }) => {
    // File watching is now handled directly in App.vue
  }

  return {
    initializeFileWatcher,
  }
}
