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

			globals.set("line", ctx.create_function(|_, (x1, y1, x2, y2, thickness, color) : (f32, f32, f32, f32, f32, Color)| {
				draw_line(x1, y1, x2, y2, thickness, color.0);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("disc", ctx.create_function(|_, (x, y, radius, color) : (f32, f32, f32, Color)| {
				draw_circle(x, y, radius, color.0);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("circle", ctx.create_function(|_, (x, y, radius, thickness, color) : (f32, f32, f32, f32, Color)| {
				draw_circle_lines(x, y, radius, thickness, color.0);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("draw_text", ctx.create_function(|_, (text, x, y, size, color) : (String<'_>, f32, f32, f32, Color)| {
				draw_text(text.to_str().unwrap_or_default(), x, y, size, color.0);
				Ok(())
			}).unwrap()).unwrap();
		});
	}
}

macro_rules! mq_wrapper {
	($name: ident) => {
		struct $name(macroquad::prelude::$name);
	};
}

mq_wrapper!(Color);
mq_wrapper!(Vec2);

impl<'a> FromLuaMulti<'a> for Color {
	fn from_lua_multi(values: MultiValue<'a>, lua: Context<'a>) -> Result<Self> {
		let table = match Table::from_lua_multi(values, lua) {
			Ok(table) => table,
			Err(e) => return Err(e),
		};

		let r: f32 = table.get("r").unwrap_or_default();
		let g: f32 = table.get("g").unwrap_or_default();
		let b: f32 = table.get("b").unwrap_or_default();

		Ok(Color(macroquad::prelude::Color::new(r / 255.0, g / 255.0, b / 255.0, 1.0)))
	}
}


impl<'a> FromLuaMulti<'a> for Vec2 {
	fn from_lua_multi(values: MultiValue<'a>, lua: Context<'a>) -> Result<Self> {
		let table = match Table::from_lua_multi(values, lua) {
			Ok(table) => table,
			Err(e) => return Err(e),
		};

		let x = table.get("x").unwrap_or_default();
		let y = table.get("y").unwrap_or_default();
		Ok(Vec2(vec2(x, y)))
	}
}