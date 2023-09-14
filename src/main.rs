#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anima::AnimaApp;
use macroquad::prelude::*;

#[macroquad::main("Anima")]
async fn main() {
	let mut app = AnimaApp::new();
	loop {
		app.draw();
		next_frame().await
	}
}
