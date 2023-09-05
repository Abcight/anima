pub struct AnimaApp;

impl Default for AnimaApp {
	fn default() -> Self {
		Self
	}
}

impl AnimaApp {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
		Default::default()
	}
}

impl eframe::App for AnimaApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self = self;

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("View");
			ui.hyperlink("https://github.com/abcight/anima");
			ui.add(egui::github_link_file!(
				"https://github.com/abcight/anima/blob/master/",
				"Source code."
			));
			egui::warn_if_debug_build(ui);
		});
	}
}
