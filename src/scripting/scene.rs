use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{Source, Api};

#[derive(Serialize, Deserialize)]
pub struct Scene {
	source: Source,
}

impl Scene {
	pub fn new(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
		let source = Source::new(path);
		Self {
			source,
		}
	}

	pub fn update(&mut self, api: &mut Api) {
		self.source.assert_watcher_spawned();

		if self.source.wants_reload() {
			self.source.update_content_from_file();
		}

		api.run(&self.source);
	}

	pub fn get_source(&mut self) -> &mut Source {
		&mut self.source
	}

	pub fn get_name(&self) -> &str {
		self.source
			.get_path()
			.file_name()
			.and_then(|x| x.to_str())
			.unwrap()
	}
}
