use gtk4::prelude::{BoxExt, Cast, GtkWindowExt};
use gtk4::{gio, glib, Widget};
use gtk4::{
	Box as GtkBox, Button, Entry, Image, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
	SizeGroup, SizeGroupMode, Window,
};

use crate::formula::{Formula, Term};
use crate::rowdata::{FormulaModel, FormulaObj};

pub struct SubWindow {
	// model: FormulaModel,
	pub window: Window,
}

impl SubWindow {
	pub fn new(amount: usize, max_value: usize) -> Self {
		let main_layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.margin_bottom(10)
			.margin_end(10)
			.margin_start(10)
			.margin_top(10)
			.spacing(10)
			.build();

		let scroll_window = ScrolledWindow::builder()
			.hexpand(true)
			.vexpand(true)
			.build();
		main_layout.append(&scroll_window);

		let formula_model = FormulaModel::with_opt(amount, max_value);
		let list_box = ListBox::new();
		list_box.bind_model(Some(&formula_model), move |o| {
			let f = o.downcast_ref::<FormulaObj>().expect("failed");
			Label::new(Some(&f.op().to_str())).upcast::<Widget>()
		});
		scroll_window.set_child(Some(&list_box));

		let check_btn = Button::builder()
			.label("检查结果！")
			.margin_bottom(20)
			.margin_end(20)
			.margin_start(20)
			.margin_top(20)
			.build();
		main_layout.append(&check_btn);

		let window = Window::builder()
			.title(format!("{max_value}以内加减法"))
			.default_height(400)
			.default_width(400)
			.child(&main_layout)
			.build();

		Self { window }
	}

	pub fn show(&self) {
		self.window.present();
	}
}
