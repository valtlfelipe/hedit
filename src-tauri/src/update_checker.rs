use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreBuilder;
use tokio::time::sleep;

const API_URL: &str = "https://hedit.app/api/latest-release";
const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub latest_version: String,
    pub download_url: String,
    pub current_version: String,
}

/// Parse version string (e.g., "v1.2.3" or "1.2.3") into comparable tuple
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let version = version.trim_start_matches('v');
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 3 {
        return None;
    }

    let major = parts[0].parse::<u32>().ok()?;
    let minor = parts[1].parse::<u32>().ok()?;
    let patch = parts[2].parse::<u32>().ok()?;

    Some((major, minor, patch))
}

/// Compare two versions, returns true if remote is newer than current
fn is_newer_version(current: &str, remote: &str) -> bool {
    let current_parsed = parse_version(current);
    let remote_parsed = parse_version(remote);

    match (current_parsed, remote_parsed) {
        (Some(current), Some(remote)) => remote > current,
        _ => false,
    }
}

/// Create HTTP client with appropriate headers and timeout
fn create_http_client(
    current_version: &str,
) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    reqwest::Client::builder()
        .user_agent(format!("hedit.app/{}", current_version))
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| e.into())
}

/// Fetch the latest release from the API
async fn fetch_latest_release(
    client: &reqwest::Client,
) -> Result<GitHubRelease, Box<dyn std::error::Error>> {
    let response = client.get(API_URL).send().await?;

    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()).into());
    }

    response.json().await.map_err(|e| e.into())
}

/// Create UpdateInfo from current version and release data
fn create_update_info(current_version: &str, release: &GitHubRelease) -> UpdateInfo {
    UpdateInfo {
        available: is_newer_version(current_version, &release.tag_name),
        latest_version: release.tag_name.clone(),
        download_url: release.html_url.clone(),
        current_version: current_version.to_string(),
    }
}

/// Check for updates from GitHub releases
async fn check_for_updates(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = app.package_info().version.to_string();

    let client = create_http_client(&current_version)?;
    let release = fetch_latest_release(&client).await?;
    let update_info = create_update_info(&current_version, &release);

    if update_info.available {
        app.emit("update-available", &update_info)?;
        println!(
            "Update available: {} -> {}",
            current_version, release.tag_name
        );
    } else {
        println!("No updates available. Current version: {}", current_version);
    }

    Ok(())
}

/// Manually check for updates and return the result
#[tauri::command]
pub async fn check_for_updates_manual(app: AppHandle) -> Result<UpdateInfo, String> {
    let current_version = app.package_info().version.to_string();

    let client = create_http_client(&current_version)
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let release = fetch_latest_release(&client)
        .await
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    Ok(create_update_info(&current_version, &release))
}

/// Background task that checks for updates on launch and every 24 hours
pub async fn check_updates_periodically(app: AppHandle) {
    // Disable update checking if license status is PRO_EXPIRED
    const STORE_FILENAME: &str = "settings.json";
    const STORE_KEY_LICENSE_TYPE: &str = "licenseType";
    const LICENSE_TYPE_PRO_EXPIRED: &str = "PRO_EXPIRED";

    let store = match StoreBuilder::new(&app, STORE_FILENAME)
        .disable_auto_save()
        .build()
    {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Failed to create store: {}", e);
            return;
        }
    };

    let license_type = store
        .get(STORE_KEY_LICENSE_TYPE)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    if license_type == LICENSE_TYPE_PRO_EXPIRED {
        println!("Update checking disabled: license is PRO_EXPIRED");
        return;
    }

    sleep(Duration::from_secs(5)).await; // slight delay to ensure frontend is ready

    // Initial check on app launch
    if let Err(e) = check_for_updates(app.clone()).await {
        eprintln!("Failed to check for updates: {}", e);
    }

    // Check every 24 hours
    loop {
        sleep(CHECK_INTERVAL).await;

        if let Err(e) = check_for_updates(app.clone()).await {
            eprintln!("Failed to check for updates: {}", e);
        }
    }
}
