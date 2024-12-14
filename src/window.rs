use gtk::{
	prelude::{BoxExt, ContainerExt, EntryExt, SizeGroupExt, WidgetExt},
	Box as GtkBox, Entry, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SizeGroup,
	SizeGroupMode, Window,
};

use crate::formula::{Formula, Term};

enum FormulaRowCheckResult {
	Uncheck,
	Ok,
	Err,
}

struct FormulaRow {
	container: ListBoxRow,
	formula: Formula<(Entry, usize)>,
	checkResult: FormulaRowCheckResult,
}

fn create_label(label: &str) -> Label {
	Label::builder().label(label).width_chars(4).build()
}

fn create_entry() -> Entry {
	Entry::builder().width_chars(4).build()
}

impl FormulaRow {
	fn new(formula: Formula<usize>) -> Self {
		let size_group = SizeGroup::builder().mode(SizeGroupMode::Horizontal).build();

		let main_layout = GtkBox::builder().margin(4).spacing(2).build();

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
			.spacing(10)
			.build();

		let scroll_window = ScrolledWindow::builder().build();
		main_layout.pack_start(&scroll_window, true, true, 0);

		let list_box = ListBox::new();
		scroll_window.set_child(Some(&list_box));

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
