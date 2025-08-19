use posthog_rs::Event;
use std::env;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;

/// PostHog API key for telemetry
const POSTHOG_API_KEY: &str = "phc_l1HKfDOk4sJYAC7R2e4z3xW4qk90wWXzEJWN9GE4Ykb";
/// Event name for app opening
const APP_OPENED_EVENT: &str = "app_opened";
/// Event distinct ID for system events
const SYSTEM_DISTINCT_ID: &str = "system";

/// Check if telemetry is disabled by user settings
fn is_telemetry_disabled(app_handle: &AppHandle) -> bool {
    // Disable telemetry in development mode
    if tauri::is_dev() {
        return true;
    }

    // Try to get app data directory
    let app_dir = match app_handle.path().app_data_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get app data directory: {}", e);
            return true;
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
            return true;
        }
    };

    // Check if telemetry is disabled in settings
    let is_disabled = store
        .get("disableTelemetry")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if is_disabled {
        println!("Telemetry is disabled by user settings.");
    }

    is_disabled
}

/// Send telemetry data to PostHog
pub async fn send_telemetry(handle: AppHandle) {
    // Check if telemetry is disabled
    if is_telemetry_disabled(&handle) {
        return;
    }

    // Initialize PostHog client
    let client = posthog_rs::client(POSTHOG_API_KEY).await;

    // Create event
    let mut event = Event::new(APP_OPENED_EVENT, SYSTEM_DISTINCT_ID);

    // Add properties to event
    if let Err(e) = event.insert_prop("os", env::consts::OS) {
        eprintln!("Failed to insert 'os' property: {}", e);
    }
    
    if let Err(e) = event.insert_prop("arch", env::consts::ARCH) {
        eprintln!("Failed to insert 'arch' property: {}", e);
    }
    
    if let Err(e) = event.insert_prop("version", handle.package_info().version.to_string()) {
        eprintln!("Failed to insert 'version' property: {}", e);
    }

    // Send event
    if let Err(e) = client.capture(event).await {
        eprintln!("Failed to send telemetry event: {}", e);
    }
}
