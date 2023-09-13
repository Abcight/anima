use std::path::{Path, PathBuf};

use hematita::{
	ast::{lexer::Lexer, parser},
	compiler, lua_lib,
	vm::{self, value::Function, VirtualMachine},
};
use log::error;

use serde::{Deserialize, Serialize};

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Scene {
	#[serde(skip)]
	code: Option<Function<'static>>,
	#[serde(skip)]
	vm: Option<VirtualMachine<'static>>,
	source: Source,
}

impl Scene {
	pub fn new(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
		let source = Source::new(path);
		Self {
			code: None,
			vm: None,
			source,
		}
	}

	pub fn update(&mut self, _time: f64) {
		self.source.assert_watcher_spawned();

		if self.source.wants_reload() {
			self.source.update_content_from_file();

			let lexer = Lexer {
				source: self.source.chars().peekable(),
			}
			.peekable();
			let parsed = parser::parse_block(&mut parser::TokenIterator(lexer));

			if let Err(err) = parsed {
				error!("Lua runtime error! {err}");
			} else {
				self.code = Some(compiler::compile_block(&parsed.unwrap()).into())
			}
		}

		if self.vm.is_none() {
			let global = lua_lib::standard_globals();
			self.vm = Some(vm::VirtualMachine::new(global));
		}

		let Some(vm) = &mut self.vm else { return };

		if let Some(code) = self.code.as_ref() {
			if let Err(e) = vm.execute(code, std::sync::Arc::default()) {
				error!("Lua runtime error! {e}");
			}
		}
	}

	pub fn get_source(&mut self) -> &mut Source {
		&mut self.source
	}

	pub fn get_name(&self) -> &str {
		self.source
			.get_path()
			.file_name()
			.and_then(|x| x.to_str())
			.unwrap()
	}
}
