pub mod preview;
pub use preview::Preview;

pub mod resources;
pub use resources::Resources;

pub mod editor;
pub use editor::Editor;

pub mod syntax_highlighting;

use crate::project::Project;

pub struct TabCtx<'a> {
	pub ui: &'a mut egui::Ui,
	pub mq: &'a mut miniquad::Context,
	pub project: &'a mut Project
}

pub trait Tab {
	fn ui(&mut self, ctx: TabCtx<'_>);
	fn title(&self) -> &str;
}

pub struct TabViewer<'a> {
	pub mq: &'a mut miniquad::Context,
	pub project: &'a mut Project
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
	type Tab = Box<dyn Tab>;

	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		let ctx = TabCtx {
			ui,
			project: self.project,
			mq: self.mq
		};

		tab.ui(ctx);
	}

	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.title().into()
	}
}
