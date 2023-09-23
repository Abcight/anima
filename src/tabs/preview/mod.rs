use super::{Tab, TabCtx};

use egui::*;
use macroquad::prelude::*;

mod grid;
use grid::Grid;

mod fit;
use fit::ViewFit;

#[derive(Default)]
pub struct Preview {
	target: Option<RenderTarget>,
	show_grid: bool,
	grid: Grid
}

impl Tab for Preview {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

		let Some(index) = project.current_scene_idx else {
			ui.centered_and_justified(|ui| ui.label("Preview!!! Epic!"));
			return
		};

		TopBottomPanel::top("preview_top").show_inside(ui, |ui| {
			ui.horizontal(|ui| {
				ui.group(|ui| {
					ui.label("Aspect ratio");
					int_field(ui, &mut project.ratio.0);
					int_field(ui, &mut project.ratio.1);
				});
				ui.group(|ui| {
					ui.checkbox(&mut self.show_grid, "Show grid");
				});
			});
			ui.add_space(ui.spacing().item_spacing.y);
		});
		ui.add_space(ui.spacing().item_spacing.y);

		let available_size = ui.available_size();

		if project.ratio == (0, 0) {
			project.ratio = (16, 9);
		}

		let fit = ViewFit::new(
			project.ratio.0 as f32,
			project.ratio.1 as f32,
			available_size.x,
			available_size.y,
			10.0
		);
		
		let target = self.get_update_target(&fit);

		let target_id = target
			.texture
			.raw_miniquad_texture_handle()
			.gl_internal_id();
		let egui_texture = TextureId::User(target_id as u64);

		let mut camera = Camera2D::from_display_rect(macroquad::prelude::Rect {
			x: -fit.aspect,
			y: -1.0,
			w: fit.view_space_width,
			h: -fit.view_space_height,
		});
		camera.target = macroquad::math::vec2(0.0, 0.0);
		camera.render_target = Some(*target);
		set_camera(&camera);
		clear_background(BLACK);

		let scene = &mut project.scenes[index];
		scene.update(ctx.api);

		let mut preview_rect = egui::Rect::NAN;
		ui.scope(|ui| {
			let mut min = ui.next_widget_position();
			let max_size = ui.available_size();
			let fit_size = egui::Vec2::new(fit.width - 2.0, fit.height - 2.0);
			min.x += (max_size.x - fit.width) * 0.5;
			min.y += (max_size.y - fit.height) * 0.5;
			let rect = egui::Rect::from_min_size(min, fit_size);
			ui.allocate_ui_at_rect(rect, |ui| {
				preview_rect = ui.image(egui_texture, fit_size).rect;
			});
		});

		if self.show_grid {
			self.grid.draw(ui, &fit, preview_rect);
		}
	}

	fn title(&self) -> &str {
		"Preview"
	}
}

impl Preview {
	fn get_update_target(&mut self, fit: &ViewFit) -> &mut RenderTarget {
		let target = match self.target.as_mut() {
			Some(target) => target,
			None => {
				self.target = Some(render_target(fit.width as u32, fit.height as u32));
				self.target.as_mut().unwrap()
			}
		};

		if (target.texture.width(), target.texture.height()) != (fit.width, fit.height) {
			target.delete();
			*target = render_target(fit.width as u32, fit.height as u32);
		};

		self.target.as_mut().unwrap()
	}
}

fn int_field(ui: &mut egui::Ui, value: &mut u16) -> egui::Response {
	let mut tmp_value = format!("{}", value);
	let res = TextEdit::singleline(&mut tmp_value).desired_width(30.0).show(ui).response;
	if let Ok(result) = tmp_value.parse() {
		*value = result;
	}
	res
}