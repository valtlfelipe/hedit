use std::process::Command;
use tauri::command;

#[command]
pub async fn write_system_hosts(content: String) -> Result<(), String> {
    let platform = tauri_plugin_os::platform();

    if platform == "linux" {
        return update_hosts_file_sudo(content).await;
    } else if platform == "macos" {
        return update_hosts_file(content).await;
    } else {
        return Err(format!("Unsupported platform: {}", platform));
    }
}

pub async fn update_hosts_file(content: String) -> Result<(), String> {
    std::fs::write("/etc/hosts", &content).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn update_hosts_file_sudo(content: String) -> Result<(), String> {
    // Write content to temporary file first
    let temp_path = "/tmp/hedit_hosts_update";
    std::fs::write(temp_path, &content).map_err(|e| e.to_string())?;

    // Use pkexec with cp to copy temp file to /etc/hosts
    let output = Command::new("pkexec")
        .arg("cp")
        .arg(temp_path)
        .arg("/etc/hosts")
        .output()
        .map_err(|e| e.to_string())?;

    // Clean up temp file
    let _ = std::fs::remove_file(temp_path);

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
