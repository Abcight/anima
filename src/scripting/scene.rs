use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{Source, Api};

#[derive(Serialize, Deserialize)]
pub struct Scene {
	source: Source,
	time: f64
}

impl Scene {
	pub fn new(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
		let source = Source::new(path);
		let time = 0.0;
		Self {
			source,
			time
		}
	}

	pub fn update(&mut self, api: &mut Api) {
		self.source.assert_watcher_spawned();

		if self.source.wants_reload() {
			self.source.update_content_from_file();
		}

		api.run(self.time, &self.source);
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

	pub fn get_time_mut(&mut self) -> &mut f64 {
		&mut self.time
	}
}
