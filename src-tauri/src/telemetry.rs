use posthog_rs::Event;
use std::env;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;

fn is_telemetry_disabled(app_handle: &AppHandle) -> bool {
    if tauri::is_dev() {
        return true;
    }

    let store = match app_handle.path().app_data_dir() {
        Ok(app_dir) => {
            StoreBuilder::new(&app_handle.clone(), app_dir.join("settings.json")).build()
        }
        Err(_) => return true
    };

    let store = match store {
        Ok(s) => s,
        Err(_) => return true,
    };

    let is_disabled = store
        .get("disableTelemetry")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if is_disabled {
        println!("Telemetry is disabled by user settings.");
    }

    is_disabled
}

pub async fn send_telemetry(handle: AppHandle) {
    if is_telemetry_disabled(&handle) {
        return;
    }

    let client = posthog_rs::client("phc_l1HKfDOk4sJYAC7R2e4z3xW4qk90wWXzEJWN9GE4Ykb").await;

    let mut event = Event::new("app_opened", "system");

    if event.insert_prop("os", env::consts::OS).is_err() {
        eprintln!("could not insert prop 'os' into telemetry event");
    }
    if event.insert_prop("arch", env::consts::ARCH).is_err() {
        eprintln!("could not insert prop 'arch' into telemetry event");
    }
    if event
        .insert_prop("version", handle.package_info().version.to_string())
        .is_err()
    {
        eprintln!("could not insert prop 'version' into telemetry event");
    }

    if let Err(e) = client.capture(event).await {
        eprintln!("Failed to send telemetry event: {}", e);
    }
}
