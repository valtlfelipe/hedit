use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

#[derive(Debug, Clone)]
pub enum ConfigKey {
    QuitOnClose,
    AutoUpdateHostsEnabled,
    AutoUpdateHostsInterval,
    DisableTelemetry,
}

impl ConfigKey {
    fn as_str(&self) -> &str {
        match self {
            ConfigKey::QuitOnClose => "quitOnClose",
            ConfigKey::AutoUpdateHostsEnabled => "autoUpdateHostsEnabled",
            ConfigKey::AutoUpdateHostsInterval => "autoUpdateHostsInterval",
            ConfigKey::DisableTelemetry => "disableTelemetry",
        }
    }
}

fn get_store(app_handle: &AppHandle) -> Result<Arc<Store<Wry>>, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    StoreBuilder::new(app_handle, app_dir.join("settings.json"))
        .disable_auto_save()
        .build()
        .map_err(|e| e.to_string())
}

pub fn get_settings_store_config_bool(
    app_handle: &AppHandle,
    config: ConfigKey,
    default: bool,
) -> Result<bool, String> {
    let store = get_store(app_handle)?;
    let value = store
        .get(config.as_str())
        .and_then(|v| v.as_bool())
        .unwrap_or(default);
    Ok(value)
}

pub fn get_settings_store_config_u64(
    app_handle: &AppHandle,
    config: ConfigKey,
    default: u64,
) -> Result<u64, String> {
    let store = get_store(app_handle)?;
    let value = store
        .get(config.as_str())
        .and_then(|v| v.as_u64())
        .unwrap_or(default);
    Ok(value)
}
