use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;

// TODO: The function only returns boolean values but accepts any config string. Consider using an enum for config keys to provide type safety and prevent runtime errors from typos or invalid config keys.
pub fn get_settings_store_config(app_handle: &AppHandle, config: String) -> Result<bool, String> {
    // Try to get app data directory
    let app_dir = match app_handle.path().app_data_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get app data directory: {}", e);
            return Err(e.to_string());
        }
    };

    // Try to create/load settings store
    let store = match StoreBuilder::new(app_handle, app_dir.join("settings.json"))
        .disable_auto_save()
        .build()
    {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Failed to create settings store: {}", e);
            return Err(e.to_string());
        }
    };

    // Get the requested config value
    let value = store.get(config).and_then(|v| v.as_bool()).unwrap_or(false);

    Ok(value)
}
