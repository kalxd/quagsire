use gtk::prelude::{ApplicationExt, ApplicationExtManual, WidgetExt};
use gtk::{Application, ApplicationWindow};

fn setup_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.default_height(400)
		.default_width(400)
		.title("小学二年级加减法")
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
