use super::{Tab, TabCtx};

#[derive(Default)]
pub struct Preview;

impl Preview {
	
}

impl Tab for Preview {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;
		
		let Some(index) = project.current_scene_idx else {
			ui.centered_and_justified(|ui| ui.label("Preview!!! Epic!"));
			return
		};

		let scene = &mut project.scenes[index];
		scene.update(0.0);
	}

	fn title(&self) -> &str {
		"Preview"
	}
}