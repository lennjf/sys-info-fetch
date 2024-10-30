
use gtk::{gdk::Display, gio, glib, prelude::*, Application, CssProvider};
use window::Window;


mod window;

const APP_ID: &str = "com.ning.sysInfoFetch";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("sys-info-fetch.gresource")
        .expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

pub fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}

fn load_css() {

    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}