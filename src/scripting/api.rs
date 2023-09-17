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

	pub fn run(&mut self, script: &str) {
		self.load_api();
		
		self.lua.context(|ctx| {
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
				let color = Color::new(r, g, b, 255.0);
				draw_line(x1, y1, x2, y2, thickness, color);
				Ok(())
			}).unwrap()).unwrap();

			globals
				.get::<_, Function<'_>>("__sub_fn")
				.unwrap()
				.call::<_, ()>(())
				.unwrap();
		});
	}
}