use egui_code_editor::*;

use super::{Tab, TabCtx};

pub struct Editor {
	title: String,
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			title: String::from("Editor"),
		}
	}
}

impl Tab for Editor {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

		self.title = format!(
			"Editor ({})",
			match project.loaded_scene.as_ref() {
				Some(scene) => scene.get_name(),
				None => "None",
			}
		);

		let Some(scene) = project.loaded_scene.as_mut() else { return };

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
	}

	fn title(&self) -> &str {
		&self.title
	}
}
