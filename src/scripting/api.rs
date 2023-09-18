use std::ops::{Deref, DerefMut};

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
				let color: Color = Tl(color).into();
				draw_line(x1, y1, x2, y2, thickness, color);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("line_v", ctx.create_function(|_, (a, b, thickness, color) : (Table<'_>, Table<'_>, f32, Table<'_>)| {
				let a: Vec2 = Tl(a).into();
				let b: Vec2 = Tl(b).into();
				let color = Tl(color).into();

				draw_line(a.x, a.y, b.x, b.y, thickness, color);

				Ok(())
			}).unwrap()).unwrap();

			globals.set("disc", ctx.create_function(|_, (x, y, radius, color) : (f32, f32, f32, Table<'_>)| {
				let color = Tl(color).into();
				draw_circle(x, y, radius, color);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("disc_v", ctx.create_function(|_, (o, radius, color) : (Table<'_>, f32, Table<'_>)| {
				let o: Vec2 = Tl(o).into();
				let color = Tl(color).into();
				draw_circle(o.x, o.y, radius, color);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("circle", ctx.create_function(|_, (x, y, radius, thickness, color) : (f32, f32, f32, f32, Table<'_>)| {
				let color = Tl(color).into();
				draw_circle_lines(x, y, radius, thickness, color);
				Ok(())
			}).unwrap()).unwrap();

			globals.set("circle_v", ctx.create_function(|_, (o, radius, thickness, color) : (Table<'_>, f32, f32, Table<'_>)| {
				let o: Vec2 = Tl(o).into();
				let color = Tl(color).into();
				draw_circle_lines(o.x, o.y, radius, thickness, color);
				Ok(())
			}).unwrap()).unwrap();
		});
	}
}

struct Tl<'a>(pub Table<'a>);

impl<'a> From<Table<'a>> for Tl<'a> {
    fn from(val: Table<'a>) -> Self {
		Tl(val)
    }
}

impl<'a> Deref for Tl<'a> {
	type Target = Table<'a>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> DerefMut for Tl<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
    }
}

impl From<Tl<'_>> for Vec2 {
	fn from(val: Tl<'_>) -> Self {
		let x = val.get("x").unwrap_or_default();
		let y = val.get("y").unwrap_or_default();
		vec2(x, y)
	}
}

impl From<Tl<'_>> for Color {
	fn from(val: Tl<'_>) -> Self {
		let r: f32 = val.get("r").unwrap_or_default();
		let g: f32 = val.get("g").unwrap_or_default();
		let b: f32 = val.get("b").unwrap_or_default();
		Color::new(r / 255.0, g / 255.0, b / 255.0, 1.0)
	}
}