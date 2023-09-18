use egui::{Key, Modifiers};
use egui_code_editor::*;

use super::{Tab, TabCtx};

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
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

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

		egui::ScrollArea::vertical().show(ui, |ui| {
			CodeEditor::default()
				.id_source("code editor")
				.with_rows(12)
				.with_fontsize(14.0)
				.with_theme(ColorTheme::GITHUB_DARK)
				.with_syntax(Syntax::lua())
				.with_numlines(true)
				.show(ui, scene.get_source());
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
