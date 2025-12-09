use std::time::Duration;
use tauri::{command, Manager};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::Url;

#[command]
pub async fn fetch_remote_hosts_file(
    app_handle: tauri::AppHandle,
    url: String,
    file_name: String,
) -> Result<(), String> {
    if url.is_empty() || file_name.is_empty() {
        return Err("URL or file name is empty".to_string());
    }

    let fetch_url = Url::parse(&url).map_err(|e| format!("Invalid URL: {}", e))?;

    if fetch_url.scheme() != "https" {
        return Err("Insecure URL: Only HTTPS is allowed.".to_string());
    }

    let app_version = app_handle.package_info().version.to_string();

    let client = reqwest::Client::builder()
        .user_agent(format!("hedit.app/{}", app_version))
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Error building HTTP client: {}", e))?;

    let mut response = client
        .get(fetch_url)
        .header(reqwest::header::ACCEPT, "text/plain")
        .send()
        .await
        .map_err(|e| format!("Error fetching remote hosts file: {}", e))?
        .error_for_status() // Errors on non-2xx status codes
        .map_err(|e| format!("Failed to fetch remote hosts file: {}", e))?;

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if !content_type.starts_with("text/plain") {
        return Err(format!(
            "Invalid Content-Type: expected 'text/plain', got '{}'",
            content_type
        ));
    }

    let dir = app_handle.path().app_data_dir().unwrap();
    let file_path = dir.join("files").join(&file_name);

    let mut file = File::create(&file_path)
        .await
        .map_err(|e| format!("Error creating file: {}", e))?;

    file.write_all(format!("# Fetched from: {}\n# --------------\n", url).as_bytes())
        .await
        .map_err(|e| format!("Error writing URL to file: {}", e))?;

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| format!("Error reading response body: {}", e))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Error writing to file: {}", e))?;
    }

    Ok(())
}
