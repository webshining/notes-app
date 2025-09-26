use std::collections::HashMap;

use serde::{ Deserialize, Serialize };
use serde_json::{ Map, Value };

pub struct Board {
	elements: HashMap<String, Value>,
	files: HashMap<String, Value>,
	state: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct BoardData {
	elements: Vec<Value>,
	files: Map<String, Value>,
	state: Map<String, Value>,
}

impl Board {
	pub fn new() -> Self {
		Self {
			elements: HashMap::new(),
			files: HashMap::new(),
			state: HashMap::new(),
		}
	}

	pub fn change(&mut self, data: BoardData) {
		for element in data.elements {
			let id = element.get("id").unwrap().to_string();
			self.elements.insert(id, element);
		}
		for (id, file) in data.files.into_iter() {
			self.files.insert(id, file);
		}
		for (key, value) in data.state.into_iter() {
			self.state.insert(key, value);
		}
	}

	pub fn state(&self) -> BoardData {
		let mut data = BoardData {
			elements: Vec::new(),
			files: Map::new(),
			state: Map::new(),
		};

		for (_, element) in self.elements.iter() {
			data.elements.push(element.to_owned());
		}
		for (id, file) in self.files.iter() {
			data.files.insert(id.into(), file.to_owned());
		}
		for (key, value) in self.state.iter() {
			data.state.insert(key.into(), value.to_owned());
		}

		data
	}
}

impl From<BoardData> for Board {
	fn from(data: BoardData) -> Self {
		let mut board = Board::new();

		board.change(data);

		board
	}
}
