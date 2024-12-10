use gtk::{Box as GtkBox, Frame};

pub struct Form {
	pub container: Frame,
}

impl Form {
	pub fn new(title: &str) -> Self {
		let main_layout = GtkBox::builder().build();

		let frame = Frame::builder().label(title).child(&main_layout).build();

		Self { container: frame }
	}
}
