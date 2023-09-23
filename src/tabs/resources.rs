use egui::*;

use super::{Tab, TabCtx};

#[derive(Default)]
pub struct Resources {
	scene_filter: String,
}

impl Tab for Resources {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

		ui.collapsing("Project", |ui| {
			ui.horizontal(|ui| {
				ui.label("Aspect ratio");
				ui.group(|ui| {
					int_field(ui, &mut project.ratio.0);
					int_field(ui, &mut project.ratio.1);
				});
			});
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

fn int_field(ui: &mut egui::Ui, value: &mut u16) -> egui::Response {
	let mut tmp_value = format!("{}", value);
	let res = TextEdit::singleline(&mut tmp_value)
		.desired_width(30.0)
		.show(ui)
		.response;
	if let Ok(result) = tmp_value.parse() {
		*value = result;
	}
	res
}