use egui::*;

use super::Tab;

pub struct Resources {
	component_filter: String,
	scene_filter: String
}

impl Default for Resources {
	fn default() -> Self {
		Self {
			component_filter: String::new(),
			scene_filter: String::new()
		}
	}
}

impl Tab for Resources {
	fn ui(&mut self, ui: &mut egui::Ui) {
		ui.collapsing("Components", |ui| {
			ui.add(TextEdit::singleline(&mut self.component_filter).hint_text("Search..."));
			ScrollArea::vertical().max_height(ui.available_height() / 2.0).show(ui, |ui| {

			});
		});
		ui.collapsing("Scenes", |ui| {
			ui.add(TextEdit::singleline(&mut self.scene_filter).hint_text("Search..."));
			ScrollArea::vertical().max_height(ui.available_height() / 2.0).show(ui, |ui| {

			});
		});
	}

	fn title<'a>(&self) -> &'a str {
		"Resources"
	}
}
