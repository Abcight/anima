use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Project {
	root: Option<PathBuf>
}

#[derive(Default)]
pub struct AnimaApp {
	project: Option<Project>
}

impl AnimaApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
		Default::default()
	}
}

impl AnimaApp {
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
			root: Some(project_root)
		};

		let Ok(json) = serde_json::to_string_pretty(&project) else { return };
		std::fs::write(&path, json).ok();

		self.project = Some(project);
	}

	#[cfg(target_arch = "wasm32")]
	fn new_project(&mut self) {
		todo!()
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn save_project(&self) {
		let Some(project) = &self.project else { return };

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
				Some(file) => Some((file.path().to_owned(), String::from_utf8(file.read().await).ok())),
				None => None
			}
		};

		let Some((path, Some(data))) = futures::executor::block_on(future) else { return };
		let Ok(mut project) = serde_json::from_str::<Project>(&data) else { return };

		project.root = Some(path);

		self.project = Some(project);
	}

	#[cfg(target_arch = "wasm32")]
	fn open_project(&mut self) {
		todo!()
	}
}

impl eframe::App for AnimaApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("New").clicked() {
						self.new_project();
					}
					ui.scope(|ui| {
						ui.set_enabled(self.project.is_some());
						if ui.button("Save").clicked() {
							self.save_project();
						}
					});
					if ui.button("Open").clicked() {
						self.open_project();
					}
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("View");
			ui.label(match &self.project {
				Some(project) => match &project.root {
					Some(root) => root.to_str().unwrap_or_default(),
					None => "Project open, vfs only"
				},
				None => "No project!"
			});
			ui.hyperlink("https://github.com/abcight/anima");
			ui.add(egui::github_link_file!(
				"https://github.com/abcight/anima/blob/master/",
				"Source code."
			));
			egui::warn_if_debug_build(ui);
		});

		egui::TopBottomPanel::bottom("bottom").resizable(true).show(ctx, |ui| {
			egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {

			});
		});
	}
}
