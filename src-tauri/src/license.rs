use tauri::{command, AppHandle, Emitter};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreBuilder;

/// License activation endpoint
const ACTIVATE_ENDPOINT: &str = "https://licensing.felipevm.dev/api/activate";
/// License check endpoint
const CHECK_ENDPOINT: &str = "https://licensing.felipevm.dev/api/check";
/// Product identifier
const PRODUCT_NAME: &str = "hedit";
/// Store filename
const STORE_FILENAME: &str = "settings.json";

/// License type constants
const LICENSE_TYPE_FREE: &str = "FREE";
const LICENSE_TYPE_PRO_ACTIVE: &str = "PRO_ACTIVE";
const LICENSE_TYPE_PRO_EXPIRED: &str = "PRO_EXPIRED";

/// Store key constants
const STORE_KEY_LICENSE: &str = "license";
const STORE_KEY_LICENSE_TYPE: &str = "licenseType";
const STORE_KEY_ACTIVATION_ID: &str = "activationId";
const STORE_KEY_UPDATE_EXPIRATION_DATE: &str = "updateExpirationDate";

/// Event name constants
const EVENT_LICENSE_UPDATE: &str = "license-update";
const EVENT_RELOAD_SETTINGS: &str = "reload-settings";

/// Default activation ID when not provided
const DEFAULT_ACTIVATION_ID: &str = "";

/// Represents the type of license
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LicenseType {
    Free,
    ProActive,
    ProExpired,
}

impl LicenseType {
    fn as_str(&self) -> &'static str {
        match self {
            LicenseType::Free => LICENSE_TYPE_FREE,
            LicenseType::ProActive => LICENSE_TYPE_PRO_ACTIVE,
            LicenseType::ProExpired => LICENSE_TYPE_PRO_EXPIRED,
        }
    }
}

/// Activation request payload
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ActivationRequest {
    license_key: String,
    product: String,
    is_test: bool,
    app_version: String,
    app_build_date: String,
}

/// Check request payload
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct CheckRequest {
    license_key: String,
    activation_id: String,
    product: String,
    is_test: bool,
    app_version: String,
    app_build_date: String,
}

/// Activation response from the licensing server
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivationResponse {
    success: bool,
    #[serde(default)]
    activation_id: Option<String>,
    valid_until: Option<String>,
}

/// Check response from the licensing server
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckResponse {
    #[serde(default)]
    is_active: bool,
    #[serde(default)]
    valid_until: Option<String>,
}

/// Get the build date of the application (set at compile time)
///
/// Returns the build date in YYYY-MM-DD format. If not set (e.g., in dev mode),
/// returns a fixed "dev" date to ensure consistency across runs.
fn get_build_date() -> String {
    option_env!("BUILD_DATE")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Fallback to a fixed "dev" date for development builds
            // This ensures consistency across runs and makes it clear this is a dev build
            "dev-build".to_string()
        })
}

/// Update the license type in the store
fn update_license_type(app_handle: &AppHandle, license_type: LicenseType) -> Result<(), String> {
    let store = StoreBuilder::new(app_handle, STORE_FILENAME)
        .disable_auto_save()
        .build()
        .map_err(|e| format!("Failed to create store: {}", e))?;

    store.set(STORE_KEY_LICENSE_TYPE, license_type.as_str());
    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}

fn update_valid_until(app_handle: &AppHandle, valid_until: &str) -> Result<(), String> {
    let store = StoreBuilder::new(app_handle, STORE_FILENAME)
        .disable_auto_save()
        .build()
        .map_err(|e| format!("Failed to create store: {}", e))?;

    store.set(STORE_KEY_UPDATE_EXPIRATION_DATE, valid_until.to_string());
    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;
    Ok(())
}

/// Handle an inactive license response
fn handle_inactive_license(
    app_handle: &AppHandle,
    check_response: &CheckResponse,
    build_date: &str,
) {
    let valid_until = check_response.valid_until.as_deref();

    match valid_until {
        Some(date) if date < build_date => {
            // License has expired and build date is after validUntil
            eprintln!("License is not valid for this build");
            if let Err(e) = update_license_type(app_handle, LicenseType::Free) {
                eprintln!("{}", e);
            }
            if let Err(e) = app_handle.emit(EVENT_LICENSE_UPDATE, "wrong-build") {
                eprintln!("Failed to emit license-invalid event: {}", e);
            }
        }
        Some(_) => {
            // License is still valid but expired
            eprintln!("License is expired");
            if let Err(e) = update_license_type(app_handle, LicenseType::ProExpired) {
                eprintln!("{}", e);
            }
            if let Err(e) = app_handle.emit(EVENT_LICENSE_UPDATE, "expired") {
                eprintln!("Failed to emit license-invalid event: {}", e);
            }
        }
        None => {
            // No validUntil date provided, treat as invalid
            eprintln!("License is invalid (no validUntil date)");
            if let Err(e) = update_license_type(app_handle, LicenseType::Free) {
                eprintln!("{}", e);
            }
            if let Err(e) = app_handle.emit(EVENT_LICENSE_UPDATE, "invalid") {
                eprintln!("Failed to emit license-invalid event: {}", e);
            }
        }
    }
}

/// Send a license check request to the server
async fn send_check_request(
    license_key: &str,
    activation_id: &str,
    app_handle: &AppHandle,
) -> Result<CheckResponse, String> {
    let payload = CheckRequest {
        license_key: license_key.to_string(),
        activation_id: activation_id.to_string(),
        product: PRODUCT_NAME.to_string(),
        is_test: tauri::is_dev(),
        app_version: app_handle.package_info().version.to_string(),
        app_build_date: get_build_date(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(CHECK_ENDPOINT)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Error sending license check request: {}", e))?;

    // Handle response errors
    if response.status().is_client_error() {
        let error_text = response.text().await.unwrap_or_else(|e| e.to_string());
        eprintln!("ClientError: {}", error_text);
        return Err(error_text);
    }

    if response.status().is_server_error() {
        eprintln!("ServerError: Something went wrong checking license");
        return Err("ServerError: Something went wrong".to_string());
    }

    // Parse response
    response
        .json()
        .await
        .map_err(|e| format!("Error parsing license check response: {}", e))
}

/// Send an activation request to the server
async fn send_activation_request(
    license_key: &str,
    app_handle: &AppHandle,
) -> Result<ActivationResponse, Box<dyn std::error::Error>> {
    let payload = ActivationRequest {
        license_key: license_key.to_string(),
        product: PRODUCT_NAME.to_string(),
        is_test: tauri::is_dev(),
        app_version: app_handle.package_info().version.to_string(),
        app_build_date: get_build_date(),
    };

    let client = reqwest::Client::new();
    let response = client.post(ACTIVATE_ENDPOINT).json(&payload).send().await?;

    // Handle server/client errors
    if response.status().is_client_error() {
        let error_text = response.text().await?;
        return Err(error_text.into());
    }

    if response.status().is_server_error() {
        return Err("ServerError: Something went wrong".into());
    }

    // Parse response
    response
        .json()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

/// Save license information to the store
fn save_license_to_store(
    app_handle: &AppHandle,
    license_key: &str,
    activation_id: &str,
    valid_until: &str,
) -> Result<(), String> {
    let store = StoreBuilder::new(app_handle, STORE_FILENAME)
        .disable_auto_save()
        .build()
        .map_err(|e| format!("Failed to create store: {}", e))?;

    store.set(STORE_KEY_LICENSE, license_key.to_string());
    store.set(STORE_KEY_ACTIVATION_ID, activation_id.to_string());
    store.set(STORE_KEY_UPDATE_EXPIRATION_DATE, valid_until.to_string());
    store.set(STORE_KEY_LICENSE_TYPE, LICENSE_TYPE_PRO_ACTIVE);

    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}

/// Activate a license key
#[command]
pub async fn activate(app_handle: AppHandle, license_key: String) -> Result<(), String> {
    println!("Activating license {}", license_key);
    activate_license(app_handle, &license_key)
        .await
        .map_err(|e| e.to_string())
}

/// Get the build date of the application
#[command]
pub fn get_build_date_command() -> String {
    get_build_date()
}

/// Check if the current license is valid
pub async fn check_license(app_handle: AppHandle) {
    // Load license data from store
    let store = match StoreBuilder::new(&app_handle, STORE_FILENAME)
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
    let license_type = store
        .get(STORE_KEY_LICENSE_TYPE)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| LICENSE_TYPE_FREE.to_string());

    if license_type == LICENSE_TYPE_FREE {
        println!("License is not activated");
        return;
    }

    // Extract license information
    let license_key = store
        .get(STORE_KEY_LICENSE)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let activation_id = store
        .get(STORE_KEY_ACTIVATION_ID)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    // Send check request
    let check_response = match send_check_request(&license_key, &activation_id, &app_handle).await {
        Ok(response) => response,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    update_valid_until(
        &app_handle,
        check_response.valid_until.as_deref().unwrap_or("<unknown>"),
    )
    .unwrap_or_else(|e| eprintln!("{}", e));

    // Check if license is active
    if !check_response.is_active {
        // Even if license is invalid, don't block usage - just show a warning
        // The app will continue to work in PRO_EXPIRED mode
        let build_date = get_build_date();
        handle_inactive_license(&app_handle, &check_response, &build_date);
    }
}

/// Activate a license key
async fn activate_license(
    app_handle: AppHandle,
    license_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Send activation request
    let response = send_activation_request(license_key, &app_handle).await?;

    // Check if activation was successful
    if !response.success {
        return Err("License activation failed".into());
    }

    let activation_id = response
        .activation_id
        .as_deref()
        .unwrap_or(DEFAULT_ACTIVATION_ID);
    let valid_until = response.valid_until.as_deref().unwrap_or("<unknown>");
    println!(
        "License activated with ID {} until {}",
        activation_id, valid_until
    );

    // Save license information to store
    save_license_to_store(&app_handle, license_key, activation_id, valid_until)?;

    // Notify frontend to reload settings
    if let Err(e) = app_handle.emit(EVENT_RELOAD_SETTINGS, true) {
        eprintln!("Failed to emit reload-settings event: {}", e);
    }

    Ok(())
}
