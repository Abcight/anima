use egui_dock::{NodeIndex, Tree};

use crate::{project::Project, tabs::*};
use egui_miniquad::EguiMq;
use miniquad::*;

pub struct AnimaApp {
	egui: Option<EguiMq>,
	tree: Tree<Box<dyn Tab>>,
	project: Option<Project>,
	project_dirty: bool,
}

impl AnimaApp {
	pub fn new(ctx: &mut miniquad::Context) -> Self {
		use crate::theme::*;
		let egui = EguiMq::new(ctx);
		set_theme(egui.egui_ctx(), MOCHA);

		let egui = Some(egui);

		let hierarchy = Box::<Resources>::default();
		let preview = Box::<Preview>::default();
		let editor = Box::<Editor>::default();

		let mut tree: Tree<Box<dyn Tab>> = Tree::new(vec![hierarchy]);
		let preview = tree.split_right(NodeIndex::root(), 0.2, vec![preview]);
		tree.split_below(preview[0], 0.2, vec![editor]);

		let project = None;

		Self {
			egui,
			tree,
			project,
			project_dirty: true,
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

impl miniquad::EventHandler for AnimaApp {
	fn update(&mut self, _ctx: &mut miniquad::Context) {}

	fn draw(&mut self, mq_ctx: &mut miniquad::Context) {
		mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
		mq_ctx.begin_default_pass(miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
		mq_ctx.end_render_pass();

		let dpi_scale = mq_ctx.dpi_scale();

		// draw the ui
		let mut egui = None;
		std::mem::swap(&mut self.egui, &mut egui);

		egui.as_mut().unwrap().run(mq_ctx, |_mq_ctx, egui_ctx| {
			egui_ctx.set_pixels_per_point(dpi_scale);

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

		egui.as_mut().unwrap().draw(mq_ctx);
		std::mem::swap(&mut self.egui, &mut egui);

		// draw the scene
		// [...]

		mq_ctx.commit_frame();
	}

	fn mouse_motion_event(&mut self, _: &mut Context, x: f32, y: f32) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.mouse_motion_event(x, y);
	}

	fn mouse_wheel_event(&mut self, _: &mut Context, dx: f32, dy: f32) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.mouse_wheel_event(dx, dy);
	}

	fn mouse_button_down_event(&mut self, ctx: &mut Context, mb: MouseButton, x: f32, y: f32) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.mouse_button_down_event(ctx, mb, x, y);
	}

	fn mouse_button_up_event(&mut self, ctx: &mut Context, mb: MouseButton, x: f32, y: f32) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.mouse_button_up_event(ctx, mb, x, y);
	}

	fn char_event(
		&mut self,
		_ctx: &mut Context,
		character: char,
		_keymods: KeyMods,
		_repeat: bool,
	) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.char_event(character);
	}

	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		keycode: KeyCode,
		keymods: KeyMods,
		_repeat: bool,
	) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.key_down_event(ctx, keycode, keymods);
	}

	fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
		let Some(egui) = self.egui.as_mut() else { return };
		egui.key_up_event(keycode, keymods);
	}
}
