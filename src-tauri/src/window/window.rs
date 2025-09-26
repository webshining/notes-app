use tauri::{ AppHandle, WebviewUrl, WebviewWindow };
use tauri_plugin_window_state::{ StateFlags, WindowExt };
use uuid::Uuid;

use crate::board::{ Board, BoardData };

pub struct Window {
	pub label: String,
	pub window: WebviewWindow,
	pub board: Board,
}

impl Window {
	pub fn new(app: &AppHandle, label: Option<impl AsRef<str>>, state: Option<BoardData>) -> Self {
		let board = match state {
			Some(data) => Board::from(data),
			None => Board::new(),
		};

		let label = label.map(|s| s.as_ref().into()).unwrap_or_else(|| Uuid::new_v4().to_string());
		let new_window = Self::new_window(app, &label);

		Self {
			label: label,
			window: new_window,
			board: board,
		}
	}

	fn new_window(app: &AppHandle, label: impl AsRef<str>) -> WebviewWindow {
		let window = WebviewWindow::builder(app, label.as_ref(), WebviewUrl::App("index.html".into()))
			.transparent(true)
			.shadow(false)
			.focusable(true)
			.skip_taskbar(true)
			.always_on_bottom(true)
			.decorations(false)
			.maximizable(false)
			.devtools(true)
			.build()
			.unwrap();

		window.restore_state(StateFlags::all()).unwrap();

		window
	}

	pub fn close(&self) {
		self.window.hide().unwrap();
	}
}
