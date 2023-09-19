pub struct ViewFit {
	pub width: f32,
	pub height: f32,
	pub aspect: f32,
	pub view_space_width: f32,
	pub view_space_height: f32,
	pub view_space_scale: f32
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