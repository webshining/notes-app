use std::sync::Arc;

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

mod commands;
mod windows;
use commands::*;
use windows::WindowsManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![save, get_state, close])
        .setup(|app| {
            let windows_manager = Arc::new(WindowsManager::new());
            app.manage(windows_manager.clone());
            windows_manager.load(&app.handle());

            let autostart_manager = app.autolaunch();
            let is_enabled = autostart_manager.is_enabled().unwrap_or(false);

            let add_i = MenuItem::with_id(app, "add", "Додати", true, None::<&str>)?;
            let auto_start_i = CheckMenuItem::with_id(
                app,
                "auto_start",
                "Автостарт",
                true,
                is_enabled,
                None::<&str>,
            )?;
            let quit_i = MenuItem::with_id(app, "quit", "Закрити", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&add_i, &auto_start_i, &quit_i])?;

            {
                let windows_manager = windows_manager.clone();
                TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(move |app, event| match event.id.as_ref() {
                        "add" => {
                            windows_manager.new_window(app, None);
                        }
                        "auto_start" => {
                            let autostart_manager = app.autolaunch();
                            if autostart_manager.is_enabled().unwrap_or(false) {
                                autostart_manager.disable().unwrap();
                            } else {
                                autostart_manager.enable().unwrap();
                            }
                        }
                        "quit" => {
                            windows_manager.save(app);

                            app.exit(0);
                        }
                        _ => {}
                    })
                    .build(app)?;
                Ok(())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
