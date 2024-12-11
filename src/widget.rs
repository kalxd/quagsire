use gtk::glib::object::IsA;
use gtk::prelude::{BoxExt, SizeGroupExt};
use gtk::{Box as GtkBox, Frame, Label, Orientation, SizeGroup, SizeGroupMode, Widget};

pub struct Form {
	pub container: Frame,
	main_layout: GtkBox,
	label_size_group: SizeGroup,
	widget_size_group: SizeGroup,
}

impl Form {
	pub fn new(title: &str) -> Self {
		let main_layout = GtkBox::builder().orientation(Orientation::Vertical).build();

		let frame = Frame::builder().label(title).child(&main_layout).build();

		let label_size_group = SizeGroup::builder().mode(SizeGroupMode::Horizontal).build();
		let widget_size_group = SizeGroup::builder().mode(SizeGroupMode::Horizontal).build();

		Self {
			container: frame,
			main_layout,
			label_size_group,
			widget_size_group,
		}
	}

	pub fn add_row<W: IsA<Widget>>(&self, label: &str, widget: &W) {
		let layout = GtkBox::builder().build();

		let label = Label::builder()
			.xalign(0_f32)
			.expand(true)
			.label(label)
			.build();

		self.label_size_group.add_widget(&label);
		self.widget_size_group.add_widget(widget);

		layout.pack_start(&label, false, true, 10);
		layout.pack_start(widget, true, true, 10);

		self.main_layout.pack_end(&layout, false, false, 10);
	}
}
