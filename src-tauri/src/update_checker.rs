use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;

// TODO: consider using https://hedit.app/api/latest-release
const GITHUB_API_URL: &str = "https://api.github.com/repos/valtlfelipe/hedit/releases/latest";
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

/// Check for updates from GitHub releases
async fn check_for_updates(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = app.package_info().version.to_string();

    // Create HTTP client with User-Agent header (required by GitHub API)
    let client = reqwest::Client::builder()
        .user_agent(format!("hedit.app/{}", current_version))
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(GITHUB_API_URL).send().await?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()).into());
    }

    let release: GitHubRelease = response.json().await?;

    // Check if the remote version is newer
    if is_newer_version(&current_version, &release.tag_name) {
        let update_info = UpdateInfo {
            available: true,
            latest_version: release.tag_name.clone(),
            download_url: release.html_url.clone(),
            current_version: current_version.clone(),
        };

        // Emit event to frontend
        app.emit("update-available", update_info)?;

        println!(
            "Update available: {} -> {}",
            current_version, release.tag_name
        );
    } else {
        println!("No updates available. Current version: {}", current_version);
    }

    Ok(())
}

/// Background task that checks for updates on launch and every 24 hours
pub async fn check_updates_periodically(app: AppHandle) {
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
