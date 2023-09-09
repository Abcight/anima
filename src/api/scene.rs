use std::{path::{Path, PathBuf}, ops::Deref};

use log::error;
use mlua::Lua;
use serde::{Serialize, Deserialize};

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Scene {
	#[serde(skip)]
	lua: Lua,
	source: Source
}

impl Scene {
	pub fn new(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
		let lua = Lua::new();
		let source = Source::new(path);

		Self {
			lua, source,
		}
	}

	pub fn update(&mut self) {
		self.source.assert_watcher_spawned();

		if self.source.wants_reload() {
			self.source.update_content_from_file();
		}

		if let Err(err) = self.lua.load(self.source.deref()).exec() {
			error!("Lua runtime error! {err}");
		}
	}

	pub fn get_path(&self) -> &PathBuf {
		self.source.get_path()
	}

	pub fn get_name(&self) -> &str {
		self.source.get_path().file_name().and_then(|x| x.to_str()).unwrap()
	}
}