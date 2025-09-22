use std::sync::Arc;

use serde_json::{Map, Value};
use tauri::{AppHandle, State, Window};

use crate::windows::WindowsManager;

#[tauri::command]
pub fn save(
    app: AppHandle,
    window: Window,
    windows_manager: State<'_, Arc<WindowsManager>>,
    data: Map<String, Value>,
) {
    if let Some(window) = windows_manager.get(window.label()) {
        window.change_state(data);
        windows_manager.save(&app);
    }
}
#[tauri::command]
pub fn close(app: AppHandle, window: Window, windows_manager: State<'_, Arc<WindowsManager>>) {
    let label = window.label();
    windows_manager.close(&app, label);
}
#[tauri::command]
pub fn get_state(
    window: Window,
    windows_manager: State<'_, Arc<WindowsManager>>,
) -> Map<String, Value> {
    if let Some(window) = windows_manager.get(window.label()) {
        let state = window.state.lock().unwrap().clone();
        return state;
    }

    Map::new()
}
