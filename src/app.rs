use egui_dock::{NodeIndex, Tree};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{tabs::*, api::Scene};

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
	pub scenes: Vec<Scene>,
	root_dir: Option<PathBuf>,
}

impl Project {
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

#[derive(Default)]
pub struct AnimaApp {
	tree: Tree<Box<dyn Tab>>,
	project: Option<Project>,
	project_dirty: bool
}

impl AnimaApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);

		let hierarchy = Box::<Resources>::default();
		let preview = Box::<Preview>::default();

		let mut tree: Tree<Box<dyn Tab>> = Tree::new(vec![hierarchy]);
		tree.split_right(NodeIndex::root(), 0.2, vec![preview]);

		Self {
			tree,
			project_dirty: true,
			..Default::default()
		}
	}
}

impl AnimaApp {
	fn set_project(&mut self, project: Option<Project>) {
		self.project = project;
		self.project_dirty = true;
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn new_project(&mut self) {
		let future = async {
			let file = rfd::AsyncFileDialog::new()
				.add_filter("Anima project file", &["anproj"])
				.add_filter("All files", &["*"])
				.set_directory("/")
				.save_file()
				.await;

			file.map(|file| file.path().to_owned())
		};

		let Some(path) = futures::executor::block_on(future) else { return };

		let mut project_root = path.clone();
		project_root.pop();

		let project = Project {
			root_dir: Some(project_root),
			..Default::default()
		};

		let Ok(json) = serde_json::to_string_pretty(&project) else { return };
		std::fs::write(&path, json).ok();

		self.set_project(Some(project));
	}

	#[cfg(target_arch = "wasm32")]
	fn new_project(&mut self) {
		todo!()
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn save_project(&mut self) {
		let Some(project) = &mut self.project else { return };

		let future = async {
			let file = rfd::AsyncFileDialog::new()
				.add_filter("Anima project file", &["anproj"])
				.add_filter("All files", &["*"])
				.set_directory("/")
				.save_file()
				.await;

			file.map(|file| file.path().to_owned())
		};

		let Some(path) = futures::executor::block_on(future) else { return };

		project.root_dir = Some(path.to_owned());
		self.project_dirty = true;

		let Ok(json) = serde_json::to_string_pretty(project) else { return };

		std::fs::write(path, json).ok();
	}

	#[cfg(target_arch = "wasm32")]
	fn save_project(&self) {
		todo!()
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn open_project(&mut self) {
		let future = async {
			let file = rfd::AsyncFileDialog::new()
				.add_filter("Anima project file", &["anproj"])
				.add_filter("All files", &["*"])
				.set_directory("/")
				.pick_file()
				.await;

			match file {
				Some(file) => Some((
					file.path().to_owned(),
					String::from_utf8(file.read().await).ok(),
				)),
				None => None,
			}
		};

		let Some((mut path, Some(data))) = futures::executor::block_on(future) else { return };
		let Ok(mut project) = serde_json::from_str::<Project>(&data) else { return };

		path.pop();

		project.root_dir = Some(path);
		project.assert_default_scene();

		self.set_project(Some(project));
	}

	#[cfg(target_arch = "wasm32")]
	fn open_project(&mut self) {
		todo!()
	}
}

impl eframe::App for AnimaApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("New").clicked() {
						self.new_project();
						ui.close_menu();
					}
					ui.scope(|ui| {
						ui.set_enabled(self.project.is_some());
						if ui.button("Save").clicked() {
							self.save_project();
							ui.close_menu();
						}
					});
					if ui.button("Open").clicked() {
						self.open_project();
						ui.close_menu();
					}
				});
			});
		});

		if self.project_dirty {
			let title = self.project
							.as_ref()
							.and_then(|x| match &x.root_dir {
								Some(root) => root.to_str(),
								None => Some("virtual space"),
							})
							.unwrap_or("no project");
			frame.set_window_title(&format!("Anima ({title})"));
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			if self.project.is_none() {
				ui.centered_and_justified(|ui| {
					ui.label("No project loaded!\nGo to file > new/open.")
				});
				return;
			}
			let Some(project) = self.project.as_mut() else { return };
			egui_dock::DockArea::new(&mut self.tree).show_inside(ui, &mut TabViewer { project });
		});
	}
}
