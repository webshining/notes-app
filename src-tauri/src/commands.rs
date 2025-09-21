use std::sync::Arc;

use serde_json::{json, Map, Value};
use tauri::{AppHandle, State, Window};
use tauri_plugin_store::StoreExt;

use crate::windows::WindowsManager;

#[tauri::command]
pub fn save(
    app: AppHandle,
    window: Window,
    elements: Vec<Value>,
    appState: Value,
    files: Map<String, Value>,
    readonly: bool,
) {
    let label = window.label();
    let store = app.store("store.json").unwrap();

    store.set(
        label,
        json!({"elements": elements, "appState": appState, "files": files, "readonly": readonly}),
    );
}
#[tauri::command]
pub fn close(app: AppHandle, window: Window, windows_manager: State<'_, Arc<WindowsManager>>) {
    let label = window.label();
    windows_manager.close(&app, label);
}
#[tauri::command]
pub fn get_state(app: AppHandle, window: Window) -> Option<Value> {
    let label = window.label();
    let store = app.store("store.json").unwrap();

    store.get(label)
}
