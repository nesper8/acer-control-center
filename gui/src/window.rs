use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, NavigationPage, NavigationSplitView};
use gtk::{Align, Box, Label, Orientation, PolicyType, ScrolledWindow, Stack};

use crate::battery::BatteryPage;
use crate::cooling::CoolingPage;
use crate::dashboard::Dashboard;
use crate::performance::PerformancePage;
use crate::rgb::RgbPage;
use crate::settings::SettingsPage;
use crate::widgets::sidebar::Sidebar;

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        let stack = Stack::builder()
            .hexpand(true)
            .vexpand(true)
            .transition_type(gtk::StackTransitionType::SlideLeftRight)
            .build();

        let dashboard = Dashboard::new();

        let dashboard_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&dashboard.root)
            .build();

        stack.add_titled(&dashboard_scroll, Some("dashboard"), "Dashboard");
        stack.add_titled(&RgbPage::new(), Some("rgb"), "RGB");
        stack.add_titled(&CoolingPage::new(), Some("cooling"), "Cooling");
        stack.add_titled(&PerformancePage::new(), Some("performance"), "Performance");
        stack.add_titled(&BatteryPage::new(), Some("battery"), "Battery");
        stack.add_titled(&SettingsPage::new(), Some("settings"), "Settings");

        let sidebar = Sidebar::new(&stack);

        let sidebar_page = NavigationPage::builder()
            .title("Navigation")
            .child(&sidebar)
            .build();

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        let header = HeaderBar::builder()
            .show_title(true)
            .title_widget(&Label::new(Some("Acer Control Center")))
            .build();

        content.append(&header);
        content.append(&stack);

        let navigation = NavigationSplitView::builder()
            .sidebar(&sidebar_page)
            .content(
                &NavigationPage::builder()
                    .title("Control Center")
                    .child(&content)
                    .build(),
            )
            .show_content(true)
            .build();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Acer Control Center")
            .default_width(1400)
            .default_height(850)
            .content(&navigation)
            .build();

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
