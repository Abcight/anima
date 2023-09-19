use egui::{Stroke, Color32};
use super::fit::ViewFit;

pub struct Grid {
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
	pub fn draw(&self, ui: &mut egui::Ui, fit: &ViewFit, rect: egui::Rect) {
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