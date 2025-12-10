use tauri::menu::{
    AboutMetadata, Menu, MenuEvent, MenuItemBuilder, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID,
    WINDOW_SUBMENU_ID,
};
use tauri::{AppHandle, Emitter};
use tauri::{Manager, Runtime};
use tauri_plugin_opener::OpenerExt;

/// Feedback URL for macOS menu
const FEEDBACK_URL: &str = "https://github.com/valtlfelipe/hedit/issues/new/choose";

/// Create the application menu
pub fn get_menu<R: Runtime>(app_handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let pkg_info = app_handle.package_info();
    let config = app_handle.config();
    let about_data = AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };

    let window = Submenu::with_id_and_items(
        app_handle,
        WINDOW_SUBMENU_ID,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app_handle, None)?,
            &PredefinedMenuItem::maximize(app_handle, None)?,
        ],
    )?;

    let help = Submenu::with_id_and_items(
        app_handle,
        HELP_SUBMENU_ID,
        "Help",
        true,
        &[
            #[cfg(not(target_os = "macos"))]
            &PredefinedMenuItem::about(app_handle, None, Some(about_data.clone()))?,
            #[cfg(target_os = "macos")]
            &MenuItemBuilder::with_id("open_feedback".to_string(), "Feedback").build(app_handle)?,
        ],
    )?;

    let file = Submenu::with_items(
        app_handle,
        "File",
        true,
        &[
            &MenuItemBuilder::with_id("new_file".to_string(), "New File")
                .accelerator("CmdOrCtrl+n")
                .build(app_handle)?,
            &MenuItemBuilder::with_id("new_remote_file".to_string(), "New Remote File")
                .accelerator("CmdOrCtrl+Shift+n")
                .build(app_handle)?,
            &MenuItemBuilder::with_id("activate_file".to_string(), "Activate File")
                .accelerator("CmdOrCtrl+Shift+a")
                .build(app_handle)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &MenuItemBuilder::with_id("save_file".to_string(), "Save")
                .accelerator("CmdOrCtrl+s")
                .build(app_handle)?,
        ],
    )?;

    let edit = Submenu::with_items(
        app_handle,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app_handle, None)?,
            &PredefinedMenuItem::redo(app_handle, None)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::cut(app_handle, None)?,
            &PredefinedMenuItem::copy(app_handle, None)?,
            &PredefinedMenuItem::paste(app_handle, None)?,
            &PredefinedMenuItem::select_all(app_handle, None)?,
        ],
    )?;

    let view = Submenu::with_items(
        app_handle,
        "View",
        true,
        &[
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::fullscreen(app_handle, None)?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::separator(app_handle)?,
            &MenuItemBuilder::with_id("zoom_reset".to_string(), "Reset Zoom")
                .accelerator("CmdOrCtrl+0")
                .build(app_handle)?,
            &MenuItemBuilder::with_id("zoom_in".to_string(), "Zoom In")
                .accelerator("CmdOrCtrl+=")
                .build(app_handle)?,
            &MenuItemBuilder::with_id("zoom_out".to_string(), "Zoom Out")
                .accelerator("CmdOrCtrl+-")
                .build(app_handle)?,
        ],
    )?;

    let _dev = Submenu::with_items(
        app_handle,
        "Dev",
        true,
        &[
            &MenuItemBuilder::with_id("dev_refresh".to_string(), "Refresh")
                .accelerator("CmdOrCtrl+Shift+r")
                .build(app_handle)?,
            &MenuItemBuilder::with_id("dev_toggle_devtools".to_string(), "Open Devtools")
                .accelerator("CmdOrCtrl+Shift+\\")
                .build(app_handle)?,
        ],
    )?;

    let menu = Menu::with_items(
        app_handle,
        &[
            #[cfg(target_os = "macos")]
            &Submenu::with_items(
                app_handle,
                pkg_info.name.clone(),
                true,
                &[
                    &PredefinedMenuItem::about(app_handle, None, Some(about_data))?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    // TODO:
                    // &MenuItemBuilder::with_id("settings".to_string(), "Settings")
                    //     .accelerator("CmdOrCtrl+,")
                    //     .build(app_handle)?,
                    // &MenuItemBuilder::with_id("update".to_string(), "Check for Updates")
                    //     .build(app_handle)?,
                    &MenuItemBuilder::with_id("activate_license".to_string(), "Activate License")
                        .build(app_handle)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::services(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::hide(app_handle, None)?,
                    &PredefinedMenuItem::hide_others(app_handle, None)?,
                    &PredefinedMenuItem::separator(app_handle)?,
                    &PredefinedMenuItem::quit(app_handle, None)?,
                ],
            )?,
            #[cfg(not(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            )))]
            &file,
            &edit,
            &view,
            &window,
            &help,
            #[cfg(dev)]
            &_dev,
        ],
    )?;

    Ok(menu)
}

/// Handle system menu events
pub fn handle_menu_event(app_handle: &tauri::AppHandle, event: &MenuEvent) {
    match event.id().0.as_str() {
        "new_file" => {
            if let Err(e) = app_handle.emit("new_file", true) {
                eprintln!("Failed to emit new_file event: {}", e);
            }
        }
        "new_remote_file" => {
            if let Err(e) = app_handle.emit("new_remote_file", true) {
                eprintln!("Failed to emit new_remote_file event: {}", e);
            }
        }
        "activate_file" => {
            if let Err(e) = app_handle.emit("activate_file", true) {
                eprintln!("Failed to emit activate_file event: {}", e);
            }
        }
        "save_file" => {
            if let Err(e) = app_handle.emit("save_file", true) {
                eprintln!("Failed to emit save_file event: {}", e);
            }
        }
        "activate_license" => {
            if let Err(e) = app_handle.emit("activate_license", true) {
                eprintln!("Failed to emit activate_license event: {}", e);
            }
        }
        "zoom_reset" => {
            if let Err(e) = app_handle.emit("zoom_reset", true) {
                eprintln!("Failed to emit zoom_reset event: {}", e);
            }
        }
        "zoom_in" => {
            if let Err(e) = app_handle.emit("zoom_in", true) {
                eprintln!("Failed to emit zoom_in event: {}", e);
            }
        }
        "zoom_out" => {
            if let Err(e) = app_handle.emit("zoom_out", true) {
                eprintln!("Failed to emit zoom_out event: {}", e);
            }
        }
        "open_feedback" => {
            if let Err(e) = app_handle.opener().open_url(FEEDBACK_URL, None::<&str>) {
                eprintln!("Failed to open feedback URL: {}", e);
            }
        }
        #[cfg(dev)]
        "dev_refresh" => {
            if let Some(window) = app_handle.get_webview_window("main") {
                if let Err(e) = window.eval("location.reload()") {
                    eprintln!("Failed to refresh window: {}", e);
                }
            } else {
                eprintln!("Failed to get main window for refresh");
            }
        }
        #[cfg(dev)]
        "dev_toggle_devtools" => {
            if let Some(window) = app_handle.get_webview_window("main") {
                if window.is_devtools_open() {
                    window.close_devtools();
                } else {
                    window.open_devtools();
                }
            } else {
                eprintln!("Failed to get main window for devtools");
            }
        }
        _ => eprintln!("Unexpected menu event: {}", event.id().0),
    }
}

/// Create the system tray menu
pub fn get_tray_menu<R: Runtime>(app_handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let tray_menu = Menu::with_items(
        app_handle,
        &[
            &MenuItemBuilder::with_id("show_app".to_string(), "Show Hedit").build(app_handle)?,
            &PredefinedMenuItem::separator(app_handle)?,
            &PredefinedMenuItem::quit(app_handle, None)?,
        ],
    )?;
    Ok(tray_menu)
}
