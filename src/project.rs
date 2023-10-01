use std::path::{PathBuf, Path};

use crate::scripting::*;
use serde::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
	#[serde(skip)]
	pub loaded_scene: Option<Scene>,
	pub scene_files_relative: Vec<PathBuf>,
	pub ratio: (u16, u16),
	root_dir: Option<PathBuf>,
	project_file: PathBuf
}

impl Project {
	pub fn get_file_path(&self) -> &Path {
		&self.project_file
	}

	pub fn set_file_path(&mut self, dir: impl AsRef<Path> + Into<PathBuf>) {
		let mut owned = dir.as_ref().to_owned();

		self.project_file = owned.clone();

		if owned.is_file() {
			owned.pop();
		}
			
		self.root_dir = Some(owned);
	}

	pub fn create_scene(&mut self, name: &str) {
		let Some(root) = self.root_dir.as_ref() else { return; };
		let mut root = root.clone();

		root.push(format!("{name}.lua"));

		if !root.exists() {
			std::fs::write(&root, "-- your first Anima scene!").ok();
		}

		let scene = Scene::new(&root);
		let relative_path = self.get_relative_path(&root).unwrap(); // TODO: Err handling

		self.scene_files_relative.push(relative_path);
		self.loaded_scene = Some(scene);
	}

	pub fn assert_default_scene(&mut self) {
		if self.scene_files_relative.is_empty() {
			self.create_scene("hello_anima");
		}
	}

	pub fn get_relative_path(&self, path: &Path) -> Result<PathBuf, ()> {
		let root = self.root_dir.as_ref().ok_or(())?;

		let root_str = root.to_string_lossy();
		let path_str = path.to_string_lossy();

		let rel_str = path_str.replace(root_str.as_ref(), "");
		
		Ok(PathBuf::from(rel_str))
	}

	pub fn get_absolute_path(&self, path: &Path) -> Result<PathBuf, ()> {
		let root = self.root_dir.as_ref().ok_or(())?;
		Ok(root.join(path))
	}

	pub fn load_scene(&mut self, path: &Path) -> Result<(), ()> {
		let relative = self.get_relative_path(path)?;
		let absolute = self.get_absolute_path(&relative)?;

		let scene = Scene::new(absolute);
		self.loaded_scene = Some(scene);

		Ok(())
	}
}
