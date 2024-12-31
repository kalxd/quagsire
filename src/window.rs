use gtk4::glib;
use gtk4::prelude::{BoxExt, ButtonExt, Cast, EditableExt, GtkWindowExt};
use gtk4::{
	Box as GtkBox, Button, Entry, Image, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
	SizeGroup, SizeGroupMode, Widget, Window,
};

use crate::rowdata::{FormulaAction, FormulaModel, FormulaObj, TermAttr};

macro_rules! term_widget {
	($obj: ident, $get_path: ident, $set_path: ident, $box: ident, $size_group: ident) => {
		match $obj.$get_path() {
			TermAttr::Value(lbl) => {
				let label = Label::builder().label(lbl.to_string().as_str()).build();
				$box.append(&label);
				$size_group.add_widget(&label);
			}
			TermAttr::Placeholder((a, b)) => {
				let entry = Entry::builder().build();
				if let Some(b) = b {
					entry.set_text(b.to_string().as_str());
				}

				$box.append(&entry);
				$size_group.add_widget(&entry);

				entry.connect_changed(glib::clone! {
					#[weak]
					$obj,
					move |entry| {
						let value = entry.text();
						let n = value.trim().parse::<usize>().ok();
						$obj.$set_path(TermAttr::Placeholder((a, n)));
					}
				});
			}
		}
	};
}

struct FormualRow {
	container: ListBoxRow,
}

impl FormualRow {
	fn new(size_group: &SizeGroup, item: &FormulaObj) -> Self {
		let main_layout = GtkBox::builder()
			.margin_start(10)
			.margin_end(10)
			.spacing(10)
			.build();
		term_widget!(item, lhs, set_lhs, main_layout, size_group);
		{
			let op = item.op().to_str();
			let label = Label::builder().width_chars(4).label(op).build();
			main_layout.append(&label);
			size_group.add_widget(&label);
		}
		term_widget!(item, rhs, set_rhs, main_layout, size_group);
		{
			let label = Label::builder().label("=").build();
			main_layout.append(&label);
		}
		term_widget!(item, result, set_result, main_layout, size_group);
		{
			let image_name = match item.action() {
				FormulaAction::Uncheck => "starred",
				FormulaAction::Fail => "window-close",
				FormulaAction::Pass => "object-select",
			};

			let icon = Image::builder().icon_name(image_name).build();
			main_layout.append(&icon);
		}

		let container = ListBoxRow::builder().child(&main_layout).build();

		Self { container }
	}
}

pub struct SubWindow {
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

		let size_group = SizeGroup::new(SizeGroupMode::Horizontal);

		let formula_model = FormulaModel::with_opt(amount, max_value);
		let list_box = ListBox::new();
		list_box.bind_model(Some(&formula_model), move |o| {
			let fo = o.downcast_ref::<FormulaObj>().expect("failed");
			let row = FormualRow::new(&size_group, fo);
			row.container.upcast::<Widget>()
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
		check_btn.connect_clicked(glib::clone! {
			#[weak]
			formula_model,
			move |_| {
				formula_model.check_result();
			}
		});

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
