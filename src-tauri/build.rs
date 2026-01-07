fn main() {
    // Set build date at compile time
    // Allow override via SOURCE_DATE_EPOCH for reproducible builds
    // or BUILD_DATE for manual override
    let build_date = if let Ok(epoch) = std::env::var("SOURCE_DATE_EPOCH") {
        // Use SOURCE_DATE_EPOCH for reproducible builds (Unix timestamp)
        match epoch.parse::<i64>() {
            Ok(timestamp) => {
                let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or_else(|| chrono::Utc::now());
                datetime.format("%Y-%m-%d").to_string()
            }
            Err(_) => {
                eprintln!("Warning: Invalid SOURCE_DATE_EPOCH, using current date");
                chrono::Utc::now().format("%Y-%m-%d").to_string()
            }
        }
    } else if let Ok(date) = std::env::var("BUILD_DATE") {
        // Allow manual override via BUILD_DATE env var
        date
    } else {
        // Default to current UTC date
        chrono::Utc::now().format("%Y-%m-%d").to_string()
    };

    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rerun-if-env-changed=BUILD_DATE");

    tauri_build::build()
}
