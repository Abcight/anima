use rlua::*;
use log::error;

use macroquad::prelude::*;
const LUA_API_SRC: &str = include_str!("api.lua");

pub struct Api {
	lua: Lua
}

impl Api {
	pub fn new() -> Self {
		let lua = Lua::new();

		Self {
			lua
		}
	}

	pub fn run(&mut self, time: f64, script: &str) {
		self.load_api();
		
		self.lua.context(|ctx| {
			let globals = ctx.globals();

			globals.set("TIME", time).unwrap();

			if let Err(e) = ctx.load(script).exec() {
				error!("Runtime Lua error! {e:?}");
			}
		});
	}

	fn load_api(&mut self) {
		self.lua.context(|ctx| {
			ctx.load(LUA_API_SRC).exec().unwrap();

			let globals = ctx.globals();

			globals.set("line", ctx.create_function(|_, (x1, y1, x2, y2, thickness, color) : (f32, f32, f32, f32, f32, Table<'_>)| {
				let r: f32 = color.get("r").unwrap_or_default();
				let g: f32 = color.get("g").unwrap_or_default();
				let b: f32 = color.get("b").unwrap_or_default();
				let color = Color::new(r / 255.0, g / 255.0, b / 255.0, 1.0);
				draw_line(x1, y1, x2, y2, thickness, color);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("line_v", ctx.create_function(|_, (a, b, thickness, color) : (Table<'_>, Table<'_>, f32, Table<'_>)| {
				let x1: f32 = a.get("x").unwrap_or_default();
				let y1: f32 = a.get("y").unwrap_or_default();
				let x2: f32 = b.get("x").unwrap_or_default();
				let y2: f32 = b.get("y").unwrap_or_default();

				let r: f32 = color.get("r").unwrap_or_default();
				let g: f32 = color.get("g").unwrap_or_default();
				let b: f32 = color.get("b").unwrap_or_default();
				let color = Color::new(r / 255.0, g / 255.0, b / 255.0, 1.0);

				draw_line(x1, y1, x2, y2, thickness, color);

				Ok(())
			}).unwrap()).unwrap();
		});
	}
}