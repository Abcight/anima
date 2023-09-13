use std::path::PathBuf;

use crate::api::*;
use serde::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
	pub scenes: Vec<Scene>,
	pub current_scene_idx: Option<usize>,
	root_dir: Option<PathBuf>,
}

impl Project {
	pub fn set_root_dir(&mut self, dir: Option<PathBuf>) {
		self.root_dir = dir;
	}

	pub fn create_scene(&mut self, name: &str) {
		let Some(root) = self.root_dir.as_ref() else { return; };
		let mut root = root.clone();

		root.push(format!("{name}.lua"));

		if !root.exists() {
			std::fs::write(&root, "-- your first Anima scene!").ok();
		}

		let scene = Scene::new(root);

		self.scenes.push(scene);
	}

	pub fn assert_default_scene(&mut self) {
		if self.scenes.is_empty() {
			self.create_scene("hello_anima");
		}
	}
}
