use gtk4::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt};
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation, SpinButton};

mod formula;
mod widget;

fn setup_ui(app: &Application) {
	let main_layout = GtkBox::builder()
		.spacing(10)
		.margin_bottom(20)
		.margin_end(20)
		.margin_start(20)
		.margin_top(20)
		.orientation(Orientation::Vertical)
		.build();

	let amount_spinbtn = SpinButton::with_range(1_f64, 1000_f64, 1_f64);
	amount_spinbtn.set_value(10_f64);

	let max_value_spinbtn = SpinButton::with_range(1_f64, 1000_f64, 1_f64);
	max_value_spinbtn.set_value(100_f64);

	let form = widget::Form::new("设置");
	form.container.set_vexpand(true);
	main_layout.append(&form.container);
	form.add_row("题目数量", &amount_spinbtn);
	form.add_row("最大值", &max_value_spinbtn);

	let submit_btn = Button::builder().label("开始做题").build();
	main_layout.append(&submit_btn);

	let window = ApplicationWindow::builder()
		.application(app)
		.title("大学二年级加减法")
		.default_height(200)
		.default_width(400)
		.child(&main_layout)
		.build();

	window.present();
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.quagsire")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}
