/// Get the build date of the application (set at compile time).
///
/// Returns the build date in YYYY-MM-DD format. If it is not set, such as in
/// development builds that do not run the build script, return a stable label.
#[tauri::command]
pub fn get_build_date_command() -> String {
    option_env!("BUILD_DATE")
        .map(str::to_string)
        .unwrap_or_else(|| "dev-build".to_string())
}
