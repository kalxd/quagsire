use gtk4::{
	prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt},
	Application, ApplicationWindow, Box as GtkBox, Button, Orientation,
};

fn setup_ui(app: &Application) {
	let main_layout = GtkBox::builder()
		.spacing(10)
		.margin_bottom(20)
		.margin_end(20)
		.margin_start(20)
		.margin_top(20)
		.orientation(Orientation::Vertical)
		.build();

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
