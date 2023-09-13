#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anima::AnimaApp;
use miniquad::*;

fn main() {
	let conf = conf::Conf {
		high_dpi: true,
		window_width: 1280,
		window_height: 720,
		window_title: String::from("Anima"),
		..Default::default()
	};

	start(conf, move |ctx| Box::new(AnimaApp::new(ctx)));
}
