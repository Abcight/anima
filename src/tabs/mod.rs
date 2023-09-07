pub mod preview;
pub use preview::Preview;

pub mod resources;
pub use resources::Resources;

pub trait Tab {
	fn ui(&mut self, ui: &mut egui::Ui);
	fn title<'a>(&self) -> &'a str;
}

pub struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
	type Tab = Box<dyn Tab>;

	fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
		tab.ui(ui);
	}

	fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
		tab.title().into()
	}
}
