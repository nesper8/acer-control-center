mod application;
mod window;

mod dashboard;
mod rgb;
mod cooling;
mod performance;
mod battery;
mod settings;

mod libacercontrol;

mod widgets;

fn main() -> glib::ExitCode {
    env_logger::init();

    adw::init().expect("Failed to initialize Libadwaita");

    application::AcerControlCenterApplication::new().run()
}
