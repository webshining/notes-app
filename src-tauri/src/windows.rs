use std::{collections::HashMap, sync::Mutex};
use tauri::{AppHandle, WebviewUrl, WebviewWindow};
use tauri_plugin_store::StoreExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};
use uuid::Uuid;

pub struct WindowsManager {
    windows: Mutex<HashMap<String, WebviewWindow>>,
}

impl WindowsManager {
    pub fn new() -> Self {
        Self {
            windows: Mutex::new(HashMap::new()),
        }
    }

    pub fn new_window(&self, app: &AppHandle, label: Option<String>) -> String {
        let label = label.unwrap_or_else(|| Uuid::new_v4().to_string());
        let window = WebviewWindow::builder(app, &label, WebviewUrl::App("index.html".into()))
            .transparent(true)
            .shadow(false)
            .focusable(true)
            .skip_taskbar(true)
            .always_on_bottom(true)
            .decorations(false)
            .maximizable(false)
            .build()
            .unwrap();

        window.restore_state(StateFlags::all()).unwrap();

        let mut windows = self.windows.lock().unwrap();
        windows.insert(label.clone(), window);

        label
    }

    pub fn load(&self, app: &AppHandle) {
        let store = app.store("store.json").unwrap();
        let labels: Vec<String> = store
            .get("labels")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        for label in labels {
            self.new_window(app, Some(label));
        }
    }

    pub fn close(&self, app: &AppHandle, label: &str) {
        let mut windows = self.windows.lock().unwrap();
        if let Some(window) = windows.remove(label) {
            let store = app.store("store.json").unwrap();
            store.delete(label);
            window.close().unwrap();

            let labels: Vec<String> = windows.keys().cloned().collect();
            store.set("labels", labels);
        }
    }

    pub fn save(&self, app: &AppHandle) {
        app.save_window_state(StateFlags::all()).unwrap();
        let labels: Vec<String> = self.windows.lock().unwrap().keys().cloned().collect();

        let store = app.store("store.json").unwrap();
        store.set("labels", labels)
    }
}
