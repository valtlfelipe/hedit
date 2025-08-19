use std::env;
mod license;
mod menu;
mod telemetry;
use std::fs::create_dir_all;

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// Background color for macOS window (RGB values)
#[cfg(target_os = "macos")]
const WINDOW_BG_COLOR: (f64, f64, f64, f64) = (50.0 / 255.0, 158.0 / 255.0, 163.5 / 255.0, 1.0);

/// Minimum window dimensions
const MIN_WINDOW_SIZE: (f64, f64) = (1000.0, 650.0);

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // Ensure app data directory exists
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data directory: {}", e))?;
            create_dir_all(&app_data_dir)
                .map_err(|e| format!("Failed to create app directory: {}", e))?;

            // Spawn background tasks
            tauri::async_runtime::spawn(telemetry::send_telemetry(app.handle().clone()));
            tauri::async_runtime::spawn(license::check_license(app.handle().clone()));

            // Setup menu
            let menu = menu::get_menu(app.handle())
                .map_err(|e| format!("Failed to create menu: {}", e))?;
            app.set_menu(menu)?;

            // Handle menu events
            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                menu::handle_menu_event(app_handle, &event);
            });

            // Create main window
            let mut win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Hedit")
                .min_inner_size(MIN_WINDOW_SIZE.0, MIN_WINDOW_SIZE.1);

            // Platform-specific window configuration
            #[cfg(target_os = "macos")]
            {
                use tauri::TitleBarStyle;
                win_builder = win_builder
                    .hidden_title(true)
                    .title_bar_style(TitleBarStyle::Overlay);
            }

            #[cfg(not(target_os = "macos"))]
            {
                win_builder = win_builder.decorations(false);
            }

            let window = win_builder
                .build()
                .map_err(|e| format!("Failed to build window: {}", e))?;

            // Platform-specific window customization
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id as cocoa_id, nil};

                let ns_window = window
                    .ns_window()
                    .map_err(|e| format!("Failed to get NSWindow: {}", e))?
                    as cocoa_id;

                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        WINDOW_BG_COLOR.0,
                        WINDOW_BG_COLOR.1,
                        WINDOW_BG_COLOR.2,
                        WINDOW_BG_COLOR.3,
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![license::activate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
