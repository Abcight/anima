use std::ops::DerefMut;

use egui::{Key, Modifiers};

use super::Tab;

pub struct Editor {
	scene_idx: Option<usize>,
	title: String,
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			scene_idx: Default::default(),
			title: String::from("Editor"),
		}
	}
}

impl Tab for Editor {
	fn ui(&mut self, ui: &mut egui::Ui, project: &mut crate::project::Project) {
		if project.current_scene_idx != self.scene_idx {
			self.scene_idx = project.current_scene_idx;
			self.title = format!(
				"Editor ({})",
				match self.scene_idx {
					Some(idx) => project.scenes[idx].get_name(),
					None => "None",
				}
			);
		}

		let Some(scene) = self.scene_idx.map(|x| &mut project.scenes[x]) else { return };

		let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
			let mut layout_job = super::syntax_highlighting::highlight(ui.ctx(), string, "lua");
			layout_job.wrap.max_width = wrap_width;
			ui.fonts(|f| f.layout_job(layout_job))
		};

		egui::ScrollArea::vertical().show(ui, |ui| {
			ui.add(
				egui::TextEdit::multiline(scene.get_source().deref_mut())
					.font(egui::TextStyle::Monospace) // for cursor height
					.code_editor()
					.desired_rows(10)
					.lock_focus(true)
					.desired_width(f32::INFINITY)
					.layouter(&mut layouter),
			);
		});

		ui.input_mut(|i| {
			if i.consume_key(Modifiers::CTRL, Key::S) {
				scene.get_source().save();
			}
		});
	}

	fn title(&self) -> &str {
		&self.title
	}
}
