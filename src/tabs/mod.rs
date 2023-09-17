pub mod preview;
pub use preview::Preview;

pub mod resources;
pub use resources::Resources;

pub mod editor;
pub use editor::Editor;

pub mod timeline;
pub use timeline::Timeline;

pub mod syntax_highlighting;

use crate::{project::Project, scripting::Api};

pub struct TabCtx<'a> {
	pub ui: &'a mut egui::Ui,
	pub project: &'a mut Project,
	pub api: &'a mut Api
}

pub trait Tab {
	fn ui(&mut self, ctx: TabCtx<'_>);
	fn title(&self) -> &str;
}

pub struct TabViewer<'a> {
	pub project: &'a mut Project,
	pub api: &'a mut Api
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
	type Tab = Box<dyn Tab>;

	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		let ctx = TabCtx {
			ui,
			api: self.api,
			project: self.project,
		};

		tab.ui(ctx);
	}

	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.title().into()
	}
}
