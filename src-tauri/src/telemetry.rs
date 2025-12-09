use tauri::{command, AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;
use reqwest::Client;
use serde_json::json;
use std::env;

// Disclosure: I just want bare minimum telemetry to understand usage patterns
// No personal data is collected or stored
// Telemetry can be disabled by the user

const UMAMI_SITE_ID: &str = "16b261e5-f0c5-4b24-b33d-10b7369332c5";
const UMAMI_HOST: &str = "https://sun.felipevm.dev";

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

/// Send telemetry data to Umami
pub async fn send_telemetry(handle: AppHandle, event_name: &str) {
    // Check if telemetry is disabled
    if is_telemetry_disabled(&handle) {
        return;
    }

    // Create HTTP client
    let client = Client::new();

    // Prepare event data for Umami according to their API specification
    let event_data = json!({
        "payload": {
            "hostname": "app.hedit.app",
            "language": "en-US",
            "screen": "1920x1080",
            "title": "Hedit App",
            "url": "/",
            "website": UMAMI_SITE_ID,
            "name": event_name,
            "data": {
                "os": env::consts::OS,
                "arch": env::consts::ARCH,
                "version": handle.package_info().version.to_string()
            }
        },
        "type": "event"
    });

    // Build the tracking URL
    let tracking_url = format!("{}/api/send", UMAMI_HOST);

    // Send event to Umami
    let response = client
        .post(&tracking_url)
        .header("Content-Type", "application/json")
        .json(&event_data)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("Failed to send telemetry event: HTTP {}", resp.status());
            }
        }
        Err(e) => {
            eprintln!("Failed to send telemetry event: {}", e);
        }
    }
}


#[command]
pub async fn send_telemetry_event(
    app_handle: tauri::AppHandle,
    event_name: String,
) -> Result<(), String> {
    send_telemetry(app_handle, &event_name).await;
    Ok(())
}