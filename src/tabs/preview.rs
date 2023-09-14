use super::{Tab, TabCtx};

use macroquad::prelude::*;
use egui::*;

#[derive(Default)]
pub struct Preview {
	target: Option<RenderTarget>
}

impl Tab for Preview {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;
		
		let Some(index) = project.current_scene_idx else {
			ui.centered_and_justified(|ui| ui.label("Preview!!! Epic!"));
			return
		};


		let size = ui.available_size();
		let target = match self.target.as_mut() {
			Some(target) => target,
			None => {
				self.target = Some(render_target(size.x as u32, size.y as u32));
				self.target.as_mut().unwrap()
			},
		};

		if (target.texture.width(), target.texture.height()) != (size.x, size.y) {
			target.delete();
			*target = render_target(size.x as u32, size.y as u32);
		}

		let target_id = target.texture.raw_miniquad_texture_handle().gl_internal_id();
		let egui_texture = TextureId::User(target_id as u64);

		set_camera(&Camera2D {
            zoom: macroquad::math::vec2(0.01, 0.01),
            target: macroquad::math::vec2(0.0, 0.0),
            render_target: Some(*target),
            ..Default::default()
        });
		clear_background(BLACK);
		set_default_camera();

		ui.image(egui_texture, ui.available_size());

		let scene = &mut project.scenes[index];
		scene.update(0.0);
	}

	fn title(&self) -> &str {
		"Preview"
	}
}