use serde_json::{ from_value, json };
use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use tauri_plugin_window_state::{ AppHandleExt, StateFlags };

use crate::{ board::BoardData, window::Window };

pub struct WindowManager {
	windows: HashMap<String, Window>,
}

impl WindowManager {
	pub fn new() -> Self {
		Self {
			windows: HashMap::new(),
		}
	}

	pub fn get(&self, label: String) -> Option<&Window> {
		let window = self.windows.get(&label);
		window
	}

	pub fn get_mut(&mut self, label: String) -> Option<&mut Window> {
		let window = self.windows.get_mut(&label);
		window
	}

	pub fn add(&mut self, app: &AppHandle, label: Option<String>, state: Option<BoardData>) {
		let window = Window::new(app, label, state);
		self.windows.insert(window.label.clone(), window);
	}

	pub fn load(&mut self, app: &AppHandle) {
		let store = app.store("store.json").unwrap();
		let labels = store.entries();

		for (label, state) in labels {
			self.add(app, Some(label), Some(from_value::<BoardData>(state).unwrap()));
		}
	}

	pub fn save(&self, app: &AppHandle) {
		let store = app.store("store.json").unwrap();

		for (label, window) in self.windows.iter() {
			store.set(label, json!(window.board.state()));
		}

		app.save_window_state(StateFlags::all()).unwrap();
	}

	pub fn close(&mut self, app: &AppHandle, label: String) {
		if let Some(window) = self.remove(label.clone()) {
			let store = app.store("store.json").unwrap();
			store.delete(label);
			window.close();
		}
	}

	fn remove(&mut self, label: String) -> Option<Window> {
		self.windows.remove(&label)
	}
}
