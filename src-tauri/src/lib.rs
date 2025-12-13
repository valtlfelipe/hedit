use std::env;
mod files;
mod license;
mod menu;
mod remote_hosts;
mod sync_remote_hosts;
mod telemetry;
mod update_checker;
use std::fs::create_dir_all;
use tauri::tray::TrayIconBuilder;
use tauri::{ActivationPolicy, LogicalPosition, Manager, WebviewUrl, WebviewWindowBuilder};

/// Background color for macOS window (RGB values)
#[cfg(target_os = "macos")]
const WINDOW_BG_COLOR: (f64, f64, f64, f64) = (50.0 / 255.0, 158.0 / 255.0, 163.5 / 255.0, 1.0);

/// Minimum window dimensions
const MIN_WINDOW_SIZE: (f64, f64) = (1000.0, 650.0);

fn show_app(app: &tauri::AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(ActivationPolicy::Regular);
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn hide_app(api: &tauri::CloseRequestApi, window: &tauri::Window) {
    let _ = window.hide();
    api.prevent_close();
    #[cfg(target_os = "macos")]
    {
        let _ = window
            .app_handle()
            .set_activation_policy(ActivationPolicy::Accessory);
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_app(app);
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Ensure app data directory exists
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data directory: {}", e))?;
            create_dir_all(&app_data_dir)
                .map_err(|e| format!("Failed to create app directory: {}", e))?;

            // Spawn background tasks
            tauri::async_runtime::spawn(telemetry::send_telemetry(
                app.handle().clone(),
                "app_opened",
            ));
            tauri::async_runtime::spawn(license::check_license(app.handle().clone()));
            tauri::async_runtime::spawn(update_checker::check_updates_periodically(
                app.handle().clone(),
            ));
            tauri::async_runtime::spawn(sync_remote_hosts::auto_update_hosts_periodically(
                app.handle().clone(),
            ));

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu::get_tray_menu(app.handle())?)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_app" => {
                        show_app(app);
                    }
                    _ => {}
                })
                .build(app)?;

            // Setup menu
            let menu = menu::get_menu(app.handle())
                .map_err(|e| format!("Failed to create menu: {}", e))?;

            #[cfg(not(target_os = "linux"))]
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
                    .traffic_light_position(LogicalPosition::new(12.0, 21.0))
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
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                hide_app(api, window);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            license::activate,
            files::write_system_hosts,
            remote_hosts::fetch_remote_hosts_file,
            sync_remote_hosts::trigger_manual_sync,
            telemetry::send_telemetry_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
