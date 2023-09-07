use super::Tab;

#[derive(Default)]
pub struct Preview;

impl Tab for Preview {
	fn ui(&mut self, _ui: &mut egui::Ui) {}

	fn title<'a>(&self) -> &'a str {
		"Preview"
	}
}
