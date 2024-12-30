use gtk4::glib::object::IsA;
use gtk4::prelude::{BoxExt, WidgetExt};
use gtk4::{Box as GtkBox, Frame, Label, Orientation, SizeGroup, SizeGroupMode, Widget};

pub struct Form {
	pub container: Frame,
	main_layout: GtkBox,
	label_size_group: SizeGroup,
	widget_size_group: SizeGroup,
}

impl Form {
	pub fn new(title: &str) -> Self {
		let main_layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.margin_start(10)
			.margin_end(10)
			.build();

		let frame = Frame::builder().label(title).child(&main_layout).build();

		let label_size_group = SizeGroup::new(SizeGroupMode::Horizontal);
		let widget_size_group = SizeGroup::new(SizeGroupMode::Horizontal);

		Self {
			container: frame,
			main_layout,
			label_size_group,
			widget_size_group,
		}
	}

	pub fn add_row<W: IsA<Widget> + WidgetExt>(&self, label: &str, widget: &W) {
		let layout = GtkBox::builder().spacing(20).build();

		let label = Label::builder().xalign(0_f32).label(label).build();

		self.label_size_group.add_widget(&label);

		widget.set_hexpand(true);
		self.widget_size_group.add_widget(widget);

		layout.append(&label);
		layout.append(widget);

		self.main_layout.append(&layout);
	}
}
