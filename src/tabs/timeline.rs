use super::{Tab, TabCtx};
use std::time::Duration;

#[derive(Default)]
pub struct Timeline {
	play: bool,
}

impl Tab for Timeline {
	fn ui(&mut self, ctx: TabCtx<'_>) {
		let project = ctx.project;
		let ui = ctx.ui;

		let Some(index) = project.current_scene_idx else {
			ui.centered_and_justified(|ui| ui.label("Select scene to view the timeline."));
			return
		};

		let scene = &mut project.scenes[index];
		let time = scene.get_time_mut();

		if self.play {
			*time += macroquad::prelude::get_frame_time() as f64;
		}

		ui.horizontal(|ui| {
			let duration = Duration::from_secs_f64(*time);
			let seconds = duration.as_secs() % 60;
			let minutes = (duration.as_secs() / 60) % 60;
			let hours = (duration.as_secs() / 60) / 60;
			ui.label(format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds));

			if ui
				.button(match self.play {
					true => "⏸",
					false => "▶",
				})
				.clicked()
			{
				self.play = !self.play;
			}

			let slider = egui::Slider::new(time, 0.0..=30.0)
				.smart_aim(false)
				.show_value(false);
			ui.spacing_mut().slider_width = ui.available_width();
			ui.add(slider);
		});
	}

	fn title(&self) -> &str {
		"Timeline"
	}
}
