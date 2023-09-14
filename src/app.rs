use egui_dock::{NodeIndex, Tree};

use crate::{project::Project, tabs::*};

pub struct AnimaApp {
	tree: Tree<Box<dyn Tab>>,
	project: Option<Project>,
	project_dirty: bool,
}

impl AnimaApp {
	pub fn new() -> Self {
		use crate::theme::*;

		egui_macroquad::ui(|egui_ctx| {
			set_theme(egui_ctx, MOCHA);
		});

		let hierarchy = Box::<Resources>::default();
		let preview = Box::<Preview>::default();
		let editor = Box::<Editor>::default();

		let mut tree: Tree<Box<dyn Tab>> = Tree::new(vec![hierarchy]);
		let preview = tree.split_right(NodeIndex::root(), 0.2, vec![preview]);
		tree.split_below(preview[0], 0.2, vec![editor]);

		let project = None;

		Self {
			tree,
			project,
			project_dirty: true,
		}
	}

	pub fn draw(&mut self) {
		egui_macroquad::ui(|egui_ctx| {
			egui::TopBottomPanel::top("top").show(egui_ctx, |ui| {
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

			egui::CentralPanel::default().show(egui_ctx, |ui| {
				if self.project.is_none() {
					ui.centered_and_justified(|ui| {
						ui.label("No project loaded!\nGo to file > new/open.")
					});
					return;
				}

				let Some(project) = self.project.as_mut() else { return };

				egui_dock::DockArea::new(&mut self.tree)
					.show_inside(ui, &mut TabViewer { project });
			});
		});
		
		egui_macroquad::draw();
	}

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

		let mut project = Project::default();
		project.set_root_dir(Some(project_root));

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

		project.set_root_dir(Some(path.to_owned()));
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

		project.set_root_dir(Some(path));
		project.assert_default_scene();

		self.set_project(Some(project));
	}

	#[cfg(target_arch = "wasm32")]
	fn open_project(&mut self) {
		todo!()
	}
}