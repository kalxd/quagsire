use gtk::{
	prelude::{BoxExt, ButtonExt, ContainerExt, EntryExt, ImageExt, SizeGroupExt, WidgetExt},
	Box as GtkBox, Button, Entry, Image, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
	SizeGroup, SizeGroupMode, Window,
};

use crate::formula::{Formula, Term};

fn create_label(label: &str) -> Label {
	Label::builder().label(label).width_chars(4).build()
}

fn create_entry() -> Entry {
	Entry::builder().width_chars(4).build()
}

fn check_entry_with(entry: &Entry, value: &usize) -> bool {
	let text = entry.text();
	let n = text.trim().parse::<usize>().ok();
	n.as_ref() == Some(value)
}

macro_rules! check_formula_field {
	($field: expr) => {
		let b = match $field {
			Term::Value(_) => true,
			Term::Placeholder((entry, value)) => check_entry_with(entry, value),
		};

		if !b {
			return false;
		}
	};
}

struct FormulaRow {
	container: ListBoxRow,
	icon: Image,
	formula: Formula<(Entry, usize)>,
}

impl FormulaRow {
	fn check_formula(&self) -> bool {
		check_formula_field!(&self.formula.lhs);
		check_formula_field!(&self.formula.rhs);
		check_formula_field!(&self.formula.result);
		true
	}

	fn check(&self) -> bool {
		let b = self.check_formula();
		if b {
			self.icon.set_icon_name(Some("object-select"));
		} else {
			self.icon.set_icon_name(Some("window-close"))
		}

		b
	}
}

impl FormulaRow {
	fn new(formula: Formula<usize>) -> Self {
		let size_group = SizeGroup::builder().mode(SizeGroupMode::Horizontal).build();

		let main_layout = GtkBox::builder().margin(4).spacing(2).build();

		let icon = Image::builder().icon_name("starred").build();
		main_layout.pack_end(&icon, false, false, 0);

		let row = ListBoxRow::builder().child(&main_layout).build();
		let formula: Formula<(Entry, usize)> = {
			let lhs = match formula.lhs {
				Term::Value(value) => {
					let label = create_label(&value.to_string());
					main_layout.pack_start(&label, true, false, 0);
					size_group.add_widget(&label);
					Term::Value(value)
				}
				Term::Placeholder(value) => {
					let entry = create_entry();
					main_layout.pack_start(&entry, true, false, 0);
					size_group.add_widget(&entry);
					Term::Placeholder((entry, value))
				}
			};

			let op = {
				let label = create_label(&formula.op.to_str());
				main_layout.pack_start(&label, true, false, 0);
				size_group.add_widget(&label);
				formula.op
			};

			let rhs = match formula.rhs {
				Term::Value(value) => {
					let label = create_label(&value.to_string());
					main_layout.pack_start(&label, true, false, 0);
					size_group.add_widget(&label);
					Term::Value(value)
				}
				Term::Placeholder(value) => {
					let entry = create_entry();
					main_layout.pack_start(&entry, true, false, 0);
					size_group.add_widget(&entry);
					Term::Placeholder((entry, value))
				}
			};

			let label = create_label("=");
			main_layout.pack_start(&label, true, false, 0);

			let result = match formula.result {
				Term::Value(value) => {
					let label = create_label(&value.to_string());
					main_layout.pack_start(&label, true, false, 0);
					Term::Value(value)
				}
				Term::Placeholder(value) => {
					let entry = create_entry();
					main_layout.pack_start(&entry, true, false, 0);
					size_group.add_widget(&entry);
					Term::Placeholder((entry, value))
				}
			};

			Formula {
				lhs,
				op,
				rhs,
				result,
			}
		};

		row.show();

		Self {
			container: row,
			icon,
			formula,
		}
	}
}

pub struct SubWindow {
	pub window: Window,
}

impl SubWindow {
	pub fn new(amount: usize, max_value: usize) -> Self {
		let main_layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let scroll_window = ScrolledWindow::builder().build();
		main_layout.pack_start(&scroll_window, true, true, 0);

		let list_box = ListBox::new();
		scroll_window.set_child(Some(&list_box));

		let row_list = (0..amount)
			.into_iter()
			.map(move |_| {
				let formula = Formula::new(max_value);
				FormulaRow::new(formula)
			})
			.collect::<Vec<_>>();

		row_list.iter().for_each(|x| list_box.add(&x.container));

		let check_btn = Button::builder().label("检查结果！").margin(20).build();
		main_layout.pack_start(&check_btn, false, false, 0);
		check_btn.connect_clicked(move |_| {
			Self::check_list_box(&row_list);
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
		self.window.show_all();
	}

	fn check_list_box(row_list: &[FormulaRow]) {
		for x in row_list {
			x.check();
		}
	}
}
