use gtk::{prelude::WidgetExt, Window};

use crate::formula::Formula;

pub struct SubWindow {
	pub window: Window,
}

impl SubWindow {
	pub fn new() -> Self {
		let window = Window::builder()
			.title("子窗口")
			.default_height(400)
			.default_width(400)
			.build();

		for _ in 1..10 {
			let xs = Formula::new(5);
			dbg!(xs);
		}

		Self { window }
	}

	pub fn show(&self) {
		self.window.show_all();
	}
}
