use tauri::{command, AppHandle, Emitter};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreBuilder;

/// License activation endpoint
const ACTIVATE_ENDPOINT: &str = "https://licensing.felipevm.dev/api/activate";
/// License check endpoint
const CHECK_ENDPOINT: &str = "https://licensing.felipevm.dev/api/check";
/// Product identifier
const PRODUCT_NAME: &str = "hedit";

#[command]
pub async fn activate(app_handle: AppHandle, license_key: String) -> Result<(), String> {
    println!("Activating license {}", license_key);
    activate_license(app_handle, &license_key)
        .await
        .map_err(|e| e.to_string())
}

/// Check if the current license is valid
pub async fn check_license(app_handle: AppHandle) {
    // Load license data from store
    let store = match StoreBuilder::new(&app_handle, "settings.json")
        .disable_auto_save()
        .build()
    {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Failed to create store: {}", e);
            return;
        }
    };

    // Check if app is activated
    let is_activated = store
        .get("isActivated")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_activated {
        println!("License is not activated");
        return;
    }

    // Extract license information (clone to avoid borrowing issues)
    let license_key = store
        .get("license")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let activation_id = store
        .get("activationId")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    // Prepare check payload
    let payload = serde_json::json!({
        "licenseKey": license_key,
        "activationId": activation_id,
        "product": PRODUCT_NAME,
        "isTest": tauri::is_dev(),
    });

    // Send check request
    let client = reqwest::Client::new();
    let res = match client.post(CHECK_ENDPOINT).json(&payload).send().await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error sending license check request: {}", e);
            return;
        }
    };

    // Handle response errors
    if res.status().is_client_error() {
        let error_text = match res.text().await {
            Ok(text) => text,
            Err(e) => e.to_string(),
        };
        eprintln!("ClientError: {}", error_text);
        if let Err(e) = app_handle.emit("license-invalid", true) {
            eprintln!("Failed to emit license-invalid event: {}", e);
        }
        return;
    }

    if res.status().is_server_error() {
        eprintln!("ServerError: Something went wrong checking license");
        return;
    }

    // Parse response
    let body: serde_json::Value = match res.json().await {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing license check response: {}", e);
            return;
        }
    };

    // Check if license is active
    if !body["isActive"].as_bool().unwrap_or(false) {
        if let Err(e) = app_handle.emit("license-invalid", true) {
            eprintln!("Failed to emit license-invalid event: {}", e);
        }
    }
}

/// Activate a license key
async fn activate_license(
    app_handle: AppHandle,
    license_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare activation payload
    let payload = serde_json::json!({
        "licenseKey": license_key,
        "product": PRODUCT_NAME,
        "isTest": tauri::is_dev(),
    });

    // Send activation request
    let client = reqwest::Client::new();
    let res = client.post(ACTIVATE_ENDPOINT).json(&payload).send().await?;

    // Handle server/client errors
    if res.status().is_client_error() {
        let error_text = res.text().await?;
        return Err(error_text.into());
    }

    if res.status().is_server_error() {
        return Err("ServerError: Something went wrong".into());
    }

    // Parse response
    let body: serde_json::Value = res.json().await?;

    // Check if activation was successful
    if !body["success"].as_bool().unwrap_or(false) {
        return Err("License activation failed".into());
    }

    println!(
        "License activated with ID: {}",
        body["activationId"].as_str().unwrap_or("<unknown>")
    );

    // Save license information to store
    let store = StoreBuilder::new(&app_handle, "settings.json")
        .disable_auto_save()
        .build()
        .map_err(|e| format!("Store error: {}", e))?;

    store.set("license", license_key.to_string());
    store.set(
        "activationId",
        body["activationId"].as_str().unwrap_or("<unknown>"),
    );
    store.set("isActivated", true);

    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    // Notify frontend to reload settings
    app_handle
        .emit("reload-settings", true)
        .map_err(|e| format!("Failed to emit reload-settings event: {}", e))?;

    Ok(())
}
