use gtk::glib::object::IsA;
use gtk::prelude::BoxExt;
use gtk::{Box as GtkBox, Frame, Label, Orientation, Widget};

pub struct Form {
	pub container: Frame,
	main_layout: GtkBox,
}

impl Form {
	pub fn new(title: &str) -> Self {
		let main_layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let frame = Frame::builder().label(title).child(&main_layout).build();

		Self {
			container: frame,
			main_layout,
		}
	}

	pub fn add_row<W: IsA<Widget>>(&self, label: &str, widget: &W) {
		let layout = GtkBox::builder().build();

		let label = Label::builder().label(label).build();

		layout.pack_start(&label, false, false, 10);
		layout.pack_start(widget, false, false, 10);

		self.main_layout.pack_end(&layout, false, false, 10);
	}
}
