use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::net::IpAddr;
use std::str::FromStr;
use tauri::AppHandle;
use tauri::Manager;
use tokio::fs;
use url::Url;

use crate::remote_hosts;

static HOSTNAME_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^(([a-zA-Z0-9_]|[a-zA-Z0-9_][a-zA-Z0-9_-]*[a-zA-Z0-9_])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9-]*[A-Za-z0-9])$").unwrap()
});

#[inline]
fn is_ignored_host(h: &str) -> bool {
    matches!(h, "localhost" | "broadcasthost" | "localdomain")
}

fn is_valid_ip(ip: &str) -> bool {
    IpAddr::from_str(ip).is_ok() || matches!(ip, "fe80::1%lo0")
}

fn is_valid_hostname(hostname: &str) -> bool {
    HOSTNAME_REGEX.is_match(hostname)
}

fn extract_func_param(line: &str) -> Option<(&str, &str)> {
    let func_name = line.split('(').next().unwrap_or("").trim_start_matches('@');
    let func_param = line
        .split('(')
        .nth(1)
        .and_then(|s| s.strip_suffix(')'))
        .unwrap_or("");
    if func_name.is_empty() || func_param.is_empty() {
        None
    } else {
        Some((func_name, func_param))
    }
}

fn get_local_file_path(app_handle: &AppHandle, file_name: &str) -> std::path::PathBuf {
    let dir = app_handle.path().app_data_dir().unwrap();
    dir.join("files").join(file_name.to_string() + ".hosts")
}

pub async fn validate_hosts_file(app_handle: &AppHandle, content: &str) -> Result<(), String> {
    let mut host_map: HashSet<&str> = HashSet::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();

        // Skip empty lines and comments directly.
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        if trimmed_line.starts_with('@') {
            let (func_name, func_param) = extract_func_param(trimmed_line).ok_or_else(|| {
                format!("Invalid function syntax at line {}: {}", line_num + 1, line)
            })?;

            if func_name == "remote" {
                let fetch_url = Url::parse(&func_param)
                    .map_err(|e| format!("Invalid URL at line {}: {}", line_num + 1, e))?;

                if fetch_url.scheme() != "https" {
                    return Err(format!(
                        "Insecure URL. Only HTTPS is allowed at line {}.",
                        line_num + 1
                    ));
                }
            } else if func_name == "local" {
                let file_path = get_local_file_path(app_handle, func_param);
                fs::metadata(&file_path).await.map_err(|_| {
                    format!(
                        "Local hosts file '{}' not found at line {}",
                        func_param,
                        line_num + 1
                    )
                })?;
            } else {
                return Err(format!(
                    "Unsupported function '{}' at line {}",
                    func_name,
                    line_num + 1
                ));
            }
            continue;
        }

        let mut parts = trimmed_line.split_whitespace();
        let Some(ip) = parts.next() else {
            return Err(format!(
                "Invalid line #{} in hosts file: {}",
                line_num + 1,
                trimmed_line
            ));
        };
        if !is_valid_ip(ip) {
            return Err(format!("Invalid IP at line {}: {}", line_num + 1, ip));
        }

        for (host_num, host) in parts.enumerate() {
            if host.starts_with("#") && host_num >= 1 {
                break;
            } else if host_num >= 1 {
                return Err(format!(
                    "Multiple hostnames on line {} are not supported: {}",
                    line_num + 1,
                    line
                ));
            }
            if is_ignored_host(host) {
                break;
            }
            if !is_valid_hostname(host) {
                return Err(format!(
                    "Invalid hostname at line {}: {}",
                    line_num + 1,
                    host
                ));
            }

            if host_map.contains(host) {
                return Err(format!(
                    "Duplicate hostname '{}' at line {}",
                    host,
                    line_num + 1
                ));
            } else {
                host_map.insert(host);
            }
        }
    }

    Ok(())
}

pub async fn parse_hosts_file(app_handle: &AppHandle, content: &str) -> Result<String, String> {
    // TODO: maybe we want to save to a temporary file instead of building a big string in memory?
    let mut result = String::with_capacity(content.len());

    // TODO: cache fetched remote files to avoid multiple fetches
    for (line_num, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('@') {
            let (func_name, func_param) = extract_func_param(trimmed_line).ok_or_else(|| {
                format!("Invalid function syntax at line {}: {}", line_num + 1, line)
            })?;

            if func_name == "remote" {
                let mut response = remote_hosts::fetch_remote_url(app_handle, func_param)
                    .await
                    .map_err(|e| format!("{} '{}' at line {}", e, func_param, line_num + 1,))?;

                result.push_str(&format!(
                    "# Begin included remote hosts file: {}\n",
                    func_param
                ));
                while let Some(chunk) = response.chunk().await.map_err(|e| {
                    format!(
                        "Error reading response body from remote hosts file '{}' at line {}: {}",
                        func_param,
                        line_num + 1,
                        e
                    )
                })? {
                    result.push_str(std::str::from_utf8(&chunk).map_err(|e| {
                        format!(
                            "Error decoding chunk from remote hosts file '{}' at line {}: {}",
                            func_param,
                            line_num + 1,
                            e
                        )
                    })?);
                }
                result.push('\n');
                result.push_str("# End included remote hosts file\n\n");
            } else if func_name == "local" {
                let file_path = get_local_file_path(app_handle, func_param);
                let file_content = fs::read_to_string(&file_path).await.map_err(|_| {
                    format!(
                        "Local hosts file '{}' not found at line {}",
                        func_param,
                        line_num + 1
                    )
                })?;

                result.push_str(&format!(
                    "# Begin included local hosts file: {}\n",
                    func_param
                ));
                result.push_str(&file_content);
                result.push('\n');
                result.push_str("# End included local hosts file\n\n");
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    // Remove trailing newline
    if result.ends_with('\n') {
        result.pop();
    }

    validate_hosts_file(app_handle, &result).await?;

    Ok(result)
}
