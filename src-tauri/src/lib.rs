use std::{ sync::{ Arc, RwLock } };

use tauri::{ menu::{ CheckMenuItem, Menu, MenuItem }, tray::TrayIconBuilder, Manager, WindowEvent };
use tauri_plugin_autostart::{ MacosLauncher, ManagerExt };

mod board;
mod window;

mod commands;
mod updater;

use commands::*;
use updater::update;
use window::WindowManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder
		::default()
		.plugin(
			tauri_plugin_log::Builder
				::new()
				.targets([tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout)])
				.build()
		)
		.plugin(tauri_plugin_updater::Builder::new().build())
		.plugin(tauri_plugin_single_instance::init(|_, _, _: String| {}))
		.plugin(tauri_plugin_window_state::Builder::new().build())
		.plugin(tauri_plugin_store::Builder::new().build())
		.plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
		.invoke_handler(tauri::generate_handler![save_state, get_state, close])
		.setup(|app| {
			// --- UPDATE APP ON STARTUP ---
			{
				let app = app.handle().clone();
				tauri::async_runtime::spawn(async move {
					update(app).await.unwrap();
				});
			}

			// --- SETUP APP ---
			let windows_manager = Arc::new(RwLock::new(WindowManager::new()));
			let autostart_manager = app.autolaunch();
			app.manage(windows_manager.clone());

			// --- LOAD PREV WINDOWS ---
			{
				let mut wm = windows_manager.write().unwrap();
				wm.load(app.handle());
			}

			// --- SETUP BACKGROUND SAVER ---
			{
				let app = app.handle().clone();
				let binding = windows_manager.clone();
				tauri::async_runtime::spawn(async move {
					use tokio::time::{ sleep, Duration };
					loop {
						{
							let wm = binding.write().unwrap();
							wm.save(&app);
						}
						sleep(Duration::from_secs(10)).await;
					}
				});
			}

			// --- TRAY MENU ---
			let is_enabled = autostart_manager.is_enabled().unwrap_or(false);
			let add_i = MenuItem::with_id(app, "add", "Додати", true, None::<&str>)?;
			let auto_start_i = CheckMenuItem::with_id(app, "auto_start", "Автостарт", true, is_enabled, None::<&str>)?;
			let quit_i = MenuItem::with_id(app, "quit", "Закрити", true, None::<&str>)?;
			let menu = Menu::with_items(app, &[&add_i, &auto_start_i, &quit_i])?;

			TrayIconBuilder::new()
				.icon(app.default_window_icon().unwrap().clone())
				.menu(&menu)
				.on_menu_event(move |app, event| {
					let binding = windows_manager.clone();
					let mut wm = binding.write().unwrap();
					match event.id.as_ref() {
						"add" => {
							wm.add(app, None, None);
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
							wm.save(app);

							app.exit(0);
						}
						_ => {}
					}
				})
				.build(app)?;
			Ok(())
		})
		.on_window_event(|_, event| {
			match event {
				WindowEvent::CloseRequested { api, .. } => {
					api.prevent_close();
				}
				_ => {}
			}
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
