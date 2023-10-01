use egui::{Modifiers, Key, Align2};
use egui_dock::{NodeIndex, Tree};
use egui_toast::*;

use crate::{project::Project, scripting::Api, tabs::*};

use crate::Result;

pub struct AnimaApp {
	tree: Tree<Box<dyn Tab>>,
	project: Option<Project>,
	operations: Vec<Result>,
	api: Api,
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
		let timeline = Box::<Timeline>::default();

		let mut tree: Tree<Box<dyn Tab>> = Tree::new(vec![hierarchy]);
		let preview = tree.split_right(NodeIndex::root(), 0.3, vec![preview]);
		tree.split_below(preview[0], 0.2, vec![editor]);
		tree.split_below(preview[1], 0.8, vec![timeline]);

		let project = None;
		let api = Api::new();

		let operations = Vec::new();

		macroquad::prelude::request_new_screen_size(1280.0, 720.0);

		Self {
			tree,
			project,
			operations,
			api
		}
	}

	pub fn draw(&mut self) {
		self.operations.clear();

		egui_macroquad::ui(|egui_ctx| {
			egui::TopBottomPanel::top("top").show(egui_ctx, |ui| {
				egui::menu::bar(ui, |ui| {
					ui.menu_button("File", |ui| {
						if ui.button("New").clicked() {
							let result = self.new_project();
							self.operations.push(result);
							ui.close_menu();
						}
						ui.scope(|ui| {
							ui.set_enabled(self.project.is_some());
							if ui.button("Save").clicked() {
								let result = self.save_project(true);
								self.operations.push(result);
								ui.close_menu();
							}
						});
						if ui.button("Open").clicked() {
							let result = self.open_project();
							self.operations.push(result);
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
				let api = &mut self.api;

				egui_dock::DockArea::new(&mut self.tree)
					.show_inside(ui, &mut TabViewer { project, api });

				ui.input_mut(|i| {
					if i.consume_key(Modifiers::CTRL, Key::S) {
						self.save_project(true).unwrap(); // TODO: Handle err
					}
				});
			});

			let mut toasts = Toasts::new()
			.anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
			.direction(egui::Direction::BottomUp);

			self.operations.iter().filter(|x| x.is_err()).for_each(|x| {
				toasts.add(Toast {
					kind: ToastKind::Error,
					text: x.as_ref().err().unwrap().as_str().into(),
					options: ToastOptions::default()
						.duration_in_seconds(5.0)
						.show_progress(true)
				});
			});

			toasts.show(egui_ctx);
		});

		egui_macroquad::draw();
	}

	fn set_project(&mut self, project: Option<Project>) {
		self.project = project;
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn new_project(&mut self) -> Result {
		let future = async {
			let file = rfd::AsyncFileDialog::new()
				.add_filter("Anima project file", &["anproj"])
				.add_filter("All files", &["*"])
				.set_directory("/")
				.save_file()
				.await;

			file.map(|file| file.path().to_owned())
		};

		let path = futures::executor::block_on(future);
		let path = path.ok_or("Failed selecting project file")?;

		let mut project = Project::default();
		project.set_file_path(&path);
		project.assert_default_scene();

		
		let json = serde_json::to_string_pretty(&project);
		let json = json.map_err(|x| format!("Failed serializing project:\n{}", x))?;
		self.set_project(Some(project));

		std::fs::write(&path, json).map_err(|x| format!("Failed writing project to path {:?}:\n{}", &path, x))
	}

	#[cfg(target_arch = "wasm32")]
	fn new_project(&mut self) {
		todo!()
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn save_project(&mut self, use_root: bool) -> Result {
		let project = self.project.as_mut().ok_or("No project to save")?;

		if let Some(scene) = project.loaded_scene.as_mut() {
			scene.get_source().save();
		}

		let path = match use_root {
			true => Some(project.get_file_path().to_owned()),
			false => {
				let future = async {
					let file = rfd::AsyncFileDialog::new()
						.add_filter("Anima project file", &["anproj"])
						.add_filter("All files", &["*"])
						.set_directory("/")
						.save_file()
						.await;

					file.map(|file| file.path().to_owned())
				};
				futures::executor::block_on(future)
			}
		}.ok_or("Failed selecting project file")?;

		project.set_file_path(&path);

		let json = serde_json::to_string_pretty(project);
		let json = json.map_err(|x| format!("Failed serializing project:\n{}", x))?;

		std::fs::write(path, json).ok();

		Ok(())
	}

	#[cfg(target_arch = "wasm32")]
	fn save_project(&self) {
		todo!()
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn open_project(&mut self) -> Result {
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

		let (path, data) = futures::executor::block_on(future).ok_or("Failed selecting project file")?;
		let data = data.ok_or("Couldn't read data from project file")?;
		let project = serde_json::from_str::<Project>(&data);
		let mut project = project.map_err(|x| format!("Failed deserializing project:\n{}", x))?;

		project.set_file_path(&path);
		project.assert_default_scene();

		self.set_project(Some(project));

		Ok(())
	}

	#[cfg(target_arch = "wasm32")]
	fn open_project(&mut self) {
		todo!()
	}
}

impl Default for AnimaApp {
	fn default() -> Self {
		Self::new()
	}
}
