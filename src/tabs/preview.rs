use super::Tab;

#[derive(Default)]
pub struct Preview;

impl Tab for Preview {
	fn ui(&mut self, ui: &mut egui::Ui, project: &mut crate::app::Project) {
		ui.centered_and_justified(|ui| ui.label("Preview!!! Epic!"));
	}

	fn title<'a>(&self) -> &'a str {
		"Preview"
	}
}
