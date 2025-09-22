use serde_json::{json, Map, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, WebviewUrl, WebviewWindow};
use tauri_plugin_store::StoreExt;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use uuid::Uuid;

pub struct WindowManager {
    pub window: WebviewWindow,
    pub state: Mutex<Map<String, Value>>,
}

impl WindowManager {
    pub fn new(app: &AppHandle, label: Option<String>, state: Option<Map<String, Value>>) -> Self {
        let label = label.unwrap_or_else(|| Uuid::new_v4().to_string());
        let state = state.unwrap_or_else(|| Map::new());
        let window = WebviewWindow::builder(app, &label, WebviewUrl::App("index.html".into()))
            .transparent(true)
            .shadow(false)
            .focusable(true)
            .skip_taskbar(true)
            .always_on_bottom(true)
            .decorations(false)
            .maximizable(false)
            .devtools(false)
            .build()
            .unwrap();

        Self {
            window: window,
            state: Mutex::new(state),
        }
    }

    pub fn change_state(&self, data: Map<String, Value>) {
        let mut state = self.state.lock().unwrap();

        *state = data;
    }

    pub fn close(&self) {
        self.window.hide().unwrap();
    }
}

pub struct WindowsManager {
    windows: Mutex<HashMap<String, Arc<WindowManager>>>,
}

impl WindowsManager {
    pub fn new() -> Self {
        Self {
            windows: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, label: &str) -> Option<Arc<WindowManager>> {
        let windows = self.windows.lock().unwrap();

        let window_manager = windows.get(label).cloned();

        window_manager
    }

    pub fn new_window(
        &self,
        app: &AppHandle,
        label: Option<String>,
        state: Option<Map<String, Value>>,
    ) {
        {
            let window = WindowManager::new(app, label, state);
            let mut windows = self.windows.lock().unwrap();
            windows.insert(window.window.label().to_owned(), Arc::new(window));
        }

        self.save(app);
    }

    pub fn load(&self, app: &AppHandle) {
        let store = app.store("store.json").unwrap();
        let labels = store.entries();

        for (label, state) in labels {
            if let Value::Object(state) = state {
                self.new_window(app, Some(label), Some(state));
            }
        }
    }

    pub fn close(&self, app: &AppHandle, label: &str) {
        let window_to_close = {
            let mut windows = self.windows.lock().unwrap();
            windows.remove(label)
        };

        if let Some(window) = window_to_close {
            window.close();
            self.save(app);
        }
    }

    pub fn save(&self, app: &AppHandle) {
        app.save_window_state(StateFlags::all()).unwrap();

        let windows = self.windows.lock().unwrap();

        let store = app.store("store.json").unwrap();
        store.clear();

        for (label, window) in windows.iter() {
            store.set(label, json!(window.state));
        }
    }
}
