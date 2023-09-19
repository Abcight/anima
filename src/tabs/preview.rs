use super::{Tab, TabCtx};

use egui::*;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Preview {
	target: Option<RenderTarget>,
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
				ui.label("Aspect ratio");
				int_field(ui, &mut project.ratio.0);
				int_field(ui, &mut project.ratio.1);
			});
			ui.add_space(ui.spacing().item_spacing.y);
		});

		let available_size = ui.available_size();

		if project.ratio == (0, 0) {
			project.ratio = (16, 9);
		}

		let ratio_width = project.ratio.0 as f32;
		let ratio_height = project.ratio.1 as f32;

		let (fit_x, fit_y) = get_closest_inner_rect_with_ratio(
			ratio_width,
			ratio_height,
			available_size.x,
			available_size.y
		);
		
		let target = match self.target.as_mut() {
			Some(target) => target,
			None => {
				self.target = Some(render_target(fit_x as u32, fit_y as u32));
				self.target.as_mut().unwrap()
			}
		};

		if (target.texture.width(), target.texture.height()) != (fit_x, fit_y) {
			target.delete();
			*target = render_target(fit_x as u32, fit_y as u32);
		}

		let target_id = target
			.texture
			.raw_miniquad_texture_handle()
			.gl_internal_id();
		let egui_texture = TextureId::User(target_id as u64);

		/*let mut camera = Camera2D::from_display_rect(macroquad::prelude::Rect {
			x: -(fit_x/fit_y),
			y: -1.0,
			w: (fit_x/fit_y) * 2.0,
			h: 2.0,
		});
		camera.zoom = macroquad::math::vec2(0.01, 0.01);
		camera.render_target = Some(*target);
		set_camera(&camera);*/
		/*set_camera(&Camera2D {
			zoom: macroquad::math::vec2(0.01, 0.01),
			target: macroquad::math::vec2(0.0, 0.0),
			render_target: Some(*target),
			..Default::default()
		});*/
		let aspect = fit_x / fit_y;
		let mut camera = Camera2D::from_display_rect(macroquad::prelude::Rect {
			x: -aspect,
			y: -1.0,
			w: ratio_width * 10.0,
			h: ratio_height * 10.0,
		});
		camera.target = macroquad::math::vec2(0.0, 0.0);
		camera.render_target = Some(*target);
		set_camera(&camera);
		clear_background(BLACK);

		let scene = &mut project.scenes[index];
		scene.update(ctx.api);

		ui.scope(|ui| {
			let mut min = ui.next_widget_position();
			let max_size = ui.available_size();
			let fit_size = egui::Vec2::new(fit_x - 2.0, fit_y - 2.0);
			min.x += (max_size.x - fit_x) * 0.5;
			min.y += (max_size.y - fit_y) * 0.5;
			let rect = egui::Rect::from_min_size(min, fit_size);
			ui.allocate_ui_at_rect(rect, |ui| {
				ui.image(egui_texture, fit_size);
			});
		});
	}

	fn title(&self) -> &str {
		"Preview"
	}
}

fn get_closest_inner_rect_with_ratio(ratio_width: f32, ratio_height: f32, width: f32, height: f32) -> (f32, f32) {
	let scale_w = width / ratio_width;
	let scale_h = height / ratio_height;

	let (fit_w_x, fit_w_y) = (ratio_width * scale_w, ratio_height * scale_w);
	let (fit_h_x, fit_h_y) = (ratio_width * scale_h, ratio_height * scale_h);

	if fit_w_x <= width && fit_w_y <= height {
		(fit_w_x, fit_w_y)
	} else {
		(fit_h_x, fit_h_y)
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