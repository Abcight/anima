use super::Tab;

pub struct Resources {
	items: Vec<String>,
}

impl Default for Resources {
	fn default() -> Self {
		Self {
			items: vec![
				String::from("Test1"),
				String::from("Test2"),
				String::from("Test6"),
			],
		}
	}
}

impl Tab for Resources {
	fn ui(&mut self, ui: &mut egui::Ui) {
		for item in &self.items {
			ui.label(item);
		}
	}

	fn title<'a>(&self) -> &'a str {
		"Resources"
	}
}
