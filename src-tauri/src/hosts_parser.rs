use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::net::IpAddr;
use std::str::FromStr;

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

pub fn validate_hosts_file(content: &str) -> Result<String, String> {
    let mut host_map: HashSet<&str> = HashSet::new();
    let mut result = String::with_capacity(content.len());

    for (line_num, line) in content.lines().enumerate() {
        let trimmed_line = line.trim();

        // Skip empty lines and comments directly.
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if trimmed_line.starts_with('@') {
            // TODO: add implementation for function validation
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

        result.push_str(line);
        result.push('\n');
    }

    // Remove trailing newline
    if result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}
