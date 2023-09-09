use super::Tab;

#[derive(Default)]
pub struct Preview;

impl Tab for Preview {
	fn ui(&mut self, ui: &mut egui::Ui, project: &mut crate::app::Project) {
		ui.centered_and_justified(|ui| ui.label("Preview!!! Epic!"));

		let Some(index) = project.current_scene_idx else { return };
		let scene = &mut project.scenes[index];

		scene.update(0.0);
	}

	fn title(&self) -> &str {
		"Preview"
	}
}
