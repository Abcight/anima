#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anima::AnimaApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
	env_logger::init();

	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Anima",
		native_options,
		Box::new(|cc| Box::new(AnimaApp::new(cc))),
	)
}

#[cfg(target_arch = "wasm32")]
fn main() {
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();

	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		eframe::WebRunner::new()
			.start(
				"the_canvas_id",
				web_options,
				Box::new(|cc| Box::new(AnimaApp::new(cc))),
			)
			.await
			.expect("failed to start eframe");
	});
}
