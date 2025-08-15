use tauri::{command, AppHandle, Emitter};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreBuilder;

pub type Result<T> = std::result::Result<T, String>;

#[command]
pub async fn activate(app_handle: AppHandle, license_key: &str) -> Result<()> {
    println!("Activating license {}", license_key);
    activate_license(app_handle, license_key).await
}

pub async fn check_license(app_handle: AppHandle) {
    let store = StoreBuilder::new(&app_handle.clone(), "settings.json")
        .disable_auto_save()
        .build()
        .unwrap();

    let is_activated = store
        .get("isActivated")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_activated {
        println!("License is not activated");
        return;
    }

    let license_key = store
        .get("license")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or("".to_string());

    let activation_id = store
        .get("activationId")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or("".to_string());

    let payload = serde_json::json!({
        "licenseKey": license_key.to_string(),
        "activationId": activation_id.to_string(),
        "product": "hedit",
        "isTest": tauri::is_dev(),
    });

    println!("Checking license with payload: {:?}", payload);

    let client = reqwest::Client::new();
    let res = match client
        .post("https://licensing.felipevm.dev/api/check")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("Error sending license check request: {}", e);
            return;
        }
    };

    if res.status().is_client_error() {
        let error_text = match res.text().await {
            Ok(text) => text,
            Err(e) => e.to_string(),
        };
        println!("ClientError: {}", error_text);
        app_handle.emit("license-invalid", true).unwrap();
        return;
    }

    if res.status().is_server_error() {
        println!("ServerError: Something went wrong checking license");
        return;
    }

    let body: serde_json::Value = match res.json().await {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing license check response: {}", e);
            return;
        }
    };

    if !body["isActive"].as_bool().unwrap_or(false) {
        app_handle.emit("license-invalid", true).unwrap();
    }
}

pub async fn activate_license(app_handle: AppHandle, license_key: &str) -> Result<()> {
    let payload = serde_json::json!({
        "licenseKey": license_key.to_string(),
        "product": "hedit",
        "isTest": tauri::is_dev(),
        // "app_platform": env::consts::OS.to_string(),
        // "app_version": window.app_handle().package_info().version.to_string(),
    });
    let client = reqwest::Client::new();
    let res = client
        .post("https://licensing.felipevm.dev/api/activate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_client_error() {
        return Err(res.text().await.map_err(|e| e.to_string())?);
    }

    if res.status().is_server_error() {
        return Err("ServerError: Something went wrong".to_string());
    }

    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    if !body["success"].as_bool().unwrap_or(false) {
        return Err("License activation failed".to_string());
    }

    println!(
        "License activated with ID: {}",
        body["activationId"].as_str().unwrap_or("<unknown>")
    );

    let store = StoreBuilder::new(&app_handle.clone(), "settings.json")
        .disable_auto_save()
        .build()
        .unwrap();
    store.set("license", license_key.to_string());
    store.set(
        "activationId",
        body["activationId"].as_str().unwrap_or("<unknown>"),
    );
    store.set("isActivated", true);
    store.save().unwrap();

    app_handle.emit("reload-settings", true).unwrap();

    Ok(())
}
