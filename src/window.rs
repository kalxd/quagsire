use gtk::{
	prelude::{BoxExt, ContainerExt, WidgetExt},
	Box as GtkBox, Entry, ListBox, ListBoxRow, Orientation, ScrolledWindow, Window,
};

use crate::formula::Formula;

enum FormulaRowCheckResult {
	Uncheck,
	Ok,
	Err,
}

struct FormulaRow {
	container: ListBoxRow,
	formula: Formula,
	checkResult: FormulaRowCheckResult,
}

impl FormulaRow {
	fn new(formula: Formula) -> Self {
		let main_layout = GtkBox::builder().build();

		let row = ListBoxRow::builder().child(&main_layout).build();

		Self {
			container: row,
			formula,
			checkResult: FormulaRowCheckResult::Uncheck,
		}
	}
}

pub struct SubWindow {
	pub window: Window,
	list_box: ListBox,
}

impl SubWindow {
	pub fn new(amount: usize, max_value: usize) -> Self {
		let main_layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(4)
			.build();

		let scroll_window = ScrolledWindow::builder().build();
		main_layout.pack_start(&scroll_window, true, true, 0);

		let list_box = ListBox::new();
		scroll_window.set_child(Some(&scroll_window));

		for _ in 0..amount {
			let formula = Formula::new(max_value);
			let row = FormulaRow::new(formula);
			list_box.add(&row.container);
		}

		let window = Window::builder()
			.title("子窗口")
			.default_height(400)
			.default_width(400)
			.child(&main_layout)
			.build();

		Self { window, list_box }
	}

	pub fn show(&self) {
		self.window.show_all();
	}
}
