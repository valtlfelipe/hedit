use tauri::menu::{
    AboutMetadata, Menu, MenuEvent, MenuItemBuilder, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID,
    WINDOW_SUBMENU_ID,
};
use tauri::{AppHandle, Emitter};
use tauri::{Manager, Runtime};
use tauri_plugin_opener::OpenerExt;

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

pub fn handle_menu_event(app_handle: &tauri::AppHandle, event: &MenuEvent) {
    #[cfg(dev)]
    let window = app_handle.get_webview_window("main").unwrap();

    match event.id().0.as_str() {
        "new_file" => app_handle.emit("new_file", true).unwrap(),
        "activate_file" => app_handle.emit("activate_file", true).unwrap(),
        "save_file" => app_handle.emit("save_file", true).unwrap(),
        "activate_license" => app_handle.emit("activate_license", true).unwrap(),
        "zoom_reset" => app_handle.emit("zoom_reset", true).unwrap(),
        "zoom_in" => app_handle.emit("zoom_in", true).unwrap(),
        "zoom_out" => app_handle.emit("zoom_out", true).unwrap(),
        "open_feedback" => {
            if let Err(e) = app_handle.opener().open_url(
                "https://github.com/valtlfelipe/hedit/issues/new/choose",
                None::<&str>,
            ) {
                println!("Failed to open feedback {e:?}")
            }
        }
        #[cfg(dev)]
        "dev_refresh" => window.eval("location.reload()").unwrap(),
        #[cfg(dev)]
        "dev_toggle_devtools" => {
            if window.is_devtools_open() {
                window.close_devtools();
            } else {
                window.open_devtools();
            }
        }
        _ => println!("unexpected menu event: {}", event.id().0),
    }
}
