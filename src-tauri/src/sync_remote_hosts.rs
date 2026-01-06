use crate::{files, remote_hosts::fetch_remote_url_to_file, settings_store};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{command, AppHandle, Emitter};
use tauri_plugin_store::StoreBuilder;
use tokio::time::sleep;

const SETTINGS_CHECK_INTERVAL_SECS: u64 = 3600; // 1 hour

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HostsFileMetadata {
    id: String,
    name: String,
    #[serde(rename = "type")]
    file_type: String,
    #[serde(rename = "remoteUrl")]
    remote_url: Option<String>,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "isCombo")]
    is_combo: bool,
}

/// Update all remote hosts files
async fn update_remote_hosts_files(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Load files metadata
    let files_store = match StoreBuilder::new(app, "files-metadata.json").build() {
        Ok(store) => store,
        Err(e) => {
            let _ = app.emit(
                "sync-status-update",
                serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to load files metadata: {}", e)
                }),
            );
            return Err(Box::new(e));
        }
    };

    let files_data = files_store
        .get("files")
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();

    let mut files_to_sync = Vec::new();

    for file_value in files_data {
        match serde_json::from_value::<HostsFileMetadata>(file_value.clone()) {
            Ok(file) => {
                if (file.file_type == "remote" && file.remote_url.is_some())
                    || (file.is_combo && file.is_active)
                {
                    files_to_sync.push(file);
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse hosts file metadata: {:?} - Error: {}",
                    file_value, e
                );
            }
        }
    }

    if files_to_sync.is_empty() {
        let _ = app.emit(
            "sync-status-update",
            serde_json::json!({
                "status": "idle",
                "message": "No hosts files to sync"
            }),
        );
        println!("No hosts files to update");
        return Ok(());
    }

    println!("Updating {} hosts files", files_to_sync.len());

    // Emit sync started event
    let _ = app.emit(
        "sync-status-update",
        serde_json::json!({
            "status": "in_progress",
            "message": "Syncing hosts..."
        }),
    );

    for file in files_to_sync {
        if let Some(url) = &file.remote_url {
            let file_name = format!("{}.hosts", file.id);
            match fetch_remote_url_to_file(app, url, &file_name).await {
                Ok(_) => {
                    println!("Successfully updated remote hosts file: {}", file.name);
                    // Emit event to notify frontend about the update
                    let _ = app.emit("remote-hosts-updated", &file.id);

                    // If the file is active, write it to the system hosts
                    if file.is_active {
                        files::write_system_hosts_from_file(app, &file_name)
                            .await
                            .unwrap_or_else(|e| {
                                eprintln!(
                                    "Failed to write updated hosts file '{}' to system hosts: {}",
                                    file.name, e
                                )
                            });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to update remote hosts file '{}': {}", file.name, e);
                    // Emit error status
                    let _ = app.emit(
                        "sync-status-update",
                        serde_json::json!({
                            "status": "error",
                            "message": format!("Sync failed: {}", e)
                        }),
                    );
                    continue;
                }
            }
        } else if file.is_combo {
            let file_name = format!("{}.hosts", file.id);
            files::write_system_hosts_from_file(app, &file_name)
                .await
                .unwrap_or_else(|e| {
                    eprintln!(
                        "Failed to write updated combo hosts file '{}' to system hosts: {}",
                        file.name, e
                    )
                });

            println!("Successfully updated combo hosts file: {}", file.name);
        }
    }

    // Emit sync completed event
    let _ = app.emit(
        "sync-status-update",
        serde_json::json!({
            "status": "success",
            "message": "Sync completed successfully"
        }),
    );

    Ok(())
}

/// Background task that periodically updates remote hosts files
pub async fn auto_update_hosts_periodically(app: AppHandle) {
    sleep(Duration::from_secs(10)).await; // Initial delay to ensure app is ready

    loop {
        // Load settings to check if auto-update is enabled
        let auto_update_enabled = settings_store::get_settings_store_config_bool(
            &app,
            settings_store::ConfigKey::AutoUpdateHostsEnabled,
            false,
        )
        .unwrap();

        if !auto_update_enabled {
            println!("Auto-update for hosts files is disabled");
            sleep(Duration::from_secs(SETTINGS_CHECK_INTERVAL_SECS)).await; // Check again in 1 hour
            continue;
        }

        let interval_hours = settings_store::get_settings_store_config_u64(
            &app,
            settings_store::ConfigKey::AutoUpdateHostsInterval,
            24,
        )
        .unwrap();

        let interval_duration = Duration::from_secs(interval_hours * 3600);

        // Perform the update
        if let Err(e) = update_remote_hosts_files(&app).await {
            eprintln!("Error during auto-update of hosts files: {}", e);
        }

        // Wait for the next update cycle
        sleep(interval_duration).await;
    }
}

#[command]
pub async fn trigger_manual_sync(app_handle: tauri::AppHandle) -> Result<(), String> {
    update_remote_hosts_files(&app_handle)
        .await
        .map_err(|e| e.to_string())
}
