use super::{Tab, TabCtx};

use egui::*;
use macroquad::prelude::*;

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
			h: fit.view_space_height,
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

struct ViewFit {
	width: f32,
	height: f32,
	aspect: f32,
	view_space_width: f32,
	view_space_height: f32,
	view_space_scale: f32
}

impl ViewFit {
	pub fn new(ratio_width: f32, ratio_height: f32, width: f32, height: f32, view_scale: f32) -> Self {
		let scale_w = width / ratio_width;
		let scale_h = height / ratio_height;

		let (fit_w_x, fit_w_y) = (ratio_width * scale_w, ratio_height * scale_w);
		let (fit_h_x, fit_h_y) = (ratio_width * scale_h, ratio_height * scale_h);
		let (fit_x, fit_y) = if fit_w_x <= width && fit_w_y <= height {
			(fit_w_x, fit_w_y)
		} else {
			(fit_h_x, fit_h_y)
		};

		let view_space_width = ratio_width * view_scale;
		let view_space_height = ratio_height * view_scale;
		let view_space_scale = view_scale;

		let aspect = ratio_width / ratio_height;

		Self {
			width: fit_x,
			height: fit_y,
			aspect,
			view_space_width,
			view_space_height,
			view_space_scale
		}
	}
}

struct Grid {
	thick: Stroke,
	thin: Stroke
}

impl Default for Grid {
	fn default() -> Self {
		Self {
			thick: Stroke::new(2.0, Color32::from_rgb_additive(40, 39, 77)),
			thin: Stroke::new(1.0, Color32::from_rgb_additive(40, 39, 77))
		}
	}
}

impl Grid {
	fn draw(&self, ui: &mut egui::Ui, fit: &ViewFit, rect: egui::Rect) {
		let painter = ui.painter();
		painter.rect(rect, 0.0, Color32::TRANSPARENT, self.thick);

		painter.line_segment([
			(rect.left(), rect.center().y).into(),
			(rect.right(), rect.center().y).into()],
			self.thick
		);

		painter.line_segment([
			(rect.center().x, rect.top()).into(),
			(rect.center().x, rect.bottom()).into()],
			self.thick
		);

		let mut x = 0.0;
		let x_scale = fit.width / fit.view_space_width;
		while x < fit.view_space_width {
			painter.line_segment([
				(rect.left() + x * x_scale, rect.top()).into(),
				(rect.left() + x * x_scale, rect.bottom()).into()],
				self.thin
			);
			x += fit.view_space_scale;
		}

		let mut y = fit.view_space_height / 2.0;
		let y_scale = fit.height / fit.view_space_height;
		while y < fit.view_space_height {
			painter.line_segment([
				(rect.left(), rect.top() + y * y_scale).into(),
				(rect.right(), rect.top() + y * y_scale).into()],
				self.thin
			);
			painter.line_segment([
				(rect.left(), rect.bottom() - y * y_scale).into(),
				(rect.right(), rect.bottom() - y * y_scale).into()],
				self.thin
			);
			y += fit.view_space_scale;
		}
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