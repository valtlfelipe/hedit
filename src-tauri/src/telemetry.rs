use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::json;
use std::env;
use tauri::{command, AppHandle};
use uuid::Uuid;

use crate::settings_store::{get_settings_store_config_bool, ConfigKey};

// Disclosure: I just want bare minimum telemetry to understand usage patterns
// No personal data is collected or stored
// Telemetry can be disabled by the user

const UMAMI_SITE_ID: &str = "16b261e5-f0c5-4b24-b33d-10b7369332c5";
const UMAMI_HOST: &str = "https://sun.felipevm.dev";

/// Session ID that is generated once and reused for all telemetry events
/// This ensures the same session ID is used throughout the app's lifetime
static SESSION_ID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

/// Check if telemetry is disabled by user settings
fn is_telemetry_disabled(app_handle: &AppHandle) -> bool {
    // Disable telemetry in development mode
    if tauri::is_dev() {
        return true;
    }

    // Check if telemetry is disabled in settings
    let is_disabled =
        get_settings_store_config_bool(app_handle, ConfigKey::DisableTelemetry, false)
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
            "id": &*SESSION_ID,
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
    event: String,
) -> Result<(), String> {
    send_telemetry(app_handle, &event).await;
    Ok(())
}
