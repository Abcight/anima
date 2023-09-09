use egui::*;

use crate::app::Project;

use super::Tab;

#[derive(Default)]
pub struct Resources {
	component_filter: String,
	scene_filter: String
}

impl Tab for Resources {
	fn ui(&mut self, ui: &mut egui::Ui, project: &mut Project) {
		ui.collapsing("Components", |ui| {
			ui.add(TextEdit::singleline(&mut self.component_filter).hint_text("Search..."));
			ScrollArea::vertical().max_height(ui.available_height() / 2.0).show(ui, |ui| {

			});
		});
		ui.collapsing("Scenes", |ui| {
			ui.add(TextEdit::singleline(&mut self.scene_filter).hint_text("Search..."));
			ScrollArea::vertical().max_height(ui.available_height() / 2.0).show(ui, |ui| {
				for scene in &project.scenes {
					let name = scene.get_name();

					if !name.contains(&self.scene_filter) {
						continue;
					}

					if ui.small_button(name).clicked() {
						println!("{:?}", scene.get_path());
					}
				}
			});
		});
	}

	fn title<'a>(&self) -> &'a str {
		"Resources"
	}
}
