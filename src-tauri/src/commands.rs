use std::{ sync::{ Arc, RwLock } };

use tauri::{ AppHandle, State, Window };

use crate::{ board::BoardData, window::WindowManager };

#[tauri::command]
pub fn save_state(window: Window, windows_manager: State<'_, Arc<RwLock<WindowManager>>>, state: BoardData) {
	let mut wm = windows_manager.write().unwrap();

	if let Some(window) = wm.get_mut(window.label().to_string()) {
		window.board.change(state);
	}
}

#[tauri::command]
pub fn close(app: AppHandle, window: Window, windows_manager: State<'_, Arc<RwLock<WindowManager>>>) {
	let mut wm = windows_manager.write().unwrap();
	wm.close(&app, window.label().into());
}

#[tauri::command]
pub fn get_state(window: Window, windows_manager: State<'_, Arc<RwLock<WindowManager>>>) -> Option<BoardData> {
	let wm = windows_manager.read().unwrap();

	if let Some(window) = wm.get(window.label().to_string()) {
		let state = window.board.state();
		return Some(state);
	}

	None
}
