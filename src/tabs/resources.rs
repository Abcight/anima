use egui::*;

use super::{Tab, TabCtx};

#[derive(Default)]
pub struct Resources {
	component_filter: String,
	scene_filter: String,
}

impl Tab for Resources {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

		ui.collapsing("Components", |ui| {
			ui.add(TextEdit::singleline(&mut self.component_filter).hint_text("Search..."));
			ScrollArea::vertical()
				.max_height(ui.available_height() / 2.0)
				.show(ui, |_| {});
		});
		ui.collapsing("Scenes", |ui| {
			ui.add(TextEdit::singleline(&mut self.scene_filter).hint_text("Search..."));
			ScrollArea::vertical()
				.max_height(ui.available_height() / 2.0)
				.show(ui, |ui| {
					for (index, scene) in project.scenes.iter().enumerate() {
						let name = scene.get_name();

						if !name.contains(&self.scene_filter) {
							continue;
						}

						if ui.small_button(name).clicked() {
							project.current_scene_idx = Some(index);
						}
					}
				});
		});
	}

	fn title(&self) -> &str {
		"Resources"
	}
}
