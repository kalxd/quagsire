use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt};
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation, SpinButton};

mod widget;

fn setup_ui(app: &Application) {
	let amount_spin_btn = SpinButton::with_range(0_f64, 100_f64, 1_f64);

	let form = widget::Form::new("设置");
	form.add_row("label 1", &Button::with_label("button 1dfsfffs"));
	form.add_row("label 232324242432", &Button::with_label("button 2"));

	let submit_btn = Button::with_label("开始做题");

	let main_layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.margin(20)
		.build();
	main_layout.pack_start(&amount_spin_btn, false, false, 10);
	main_layout.pack_start(&form.container, false, false, 10);
	main_layout.pack_end(&submit_btn, true, false, 0);

	let window = ApplicationWindow::builder()
		.application(app)
		.default_height(400)
		.default_width(400)
		.title("小学二年级加减法")
		.child(&main_layout)
		.build();

	window.show_all();
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.quagsire")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}
