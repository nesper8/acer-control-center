use adw::prelude::*;
use adw::Application;
use gtk::gio;
use gtk::CssProvider;
use gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

use crate::window::MainWindow;

pub struct AcerControlCenterApplication {
    app: Application,
}

impl AcerControlCenterApplication {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("io.github.stanlysilas.AcerControlCenter")
            .flags(gio::ApplicationFlags::HANDLES_OPEN)
            .build();

        let instance = Self { app };

        instance.setup_startup();
        instance.setup_activate();

        instance
    }

    pub fn run(self) -> glib::ExitCode {
        self.app.run()
    }

    fn setup_startup(&self) {
        self.app.connect_startup(|_| {
            load_css();

            log::info!("Acer Control Center started.");
        });
    }

    fn setup_activate(&self) {
        self.app.connect_activate(|app| {
            let window = MainWindow::new(app);

            window.present();

            log::info!("Main window presented.");
        });
    }
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_path("resources/css/style.css");

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("No display found."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    log::info!("CSS loaded.");
}
