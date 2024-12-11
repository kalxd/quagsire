use gtk::glib::{self, clone};
use gtk::prelude::{
	ApplicationExt, ApplicationExtManual, BoxExt, ButtonExt, GtkApplicationExt, SpinButtonExt,
	WidgetExt,
};
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation, SpinButton};

mod widget;
mod window;

fn setup_ui(app: &Application) {
	let main_layout = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(10)
		.margin(20)
		.build();

	let amount_spin_btn = SpinButton::with_range(0_f64, 100_f64, 1_f64);
	amount_spin_btn.set_value(10_f64);

	let form = widget::Form::new("设置");

	main_layout.pack_start(&form.container, false, false, 10);

	form.add_row("题目数量", &amount_spin_btn);

	let submit_btn = Button::with_label("开始做题");
	main_layout.pack_end(&submit_btn, true, false, 0);
	submit_btn.connect_clicked(clone!(@weak app => move |_| {
		let sub_window = window::SubWindow::new();
		app.add_window(&sub_window.window);

		sub_window.show();
	}));

	let window = ApplicationWindow::builder()
		.application(app)
		.default_height(200)
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
