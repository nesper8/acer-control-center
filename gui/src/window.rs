use adw::prelude::*;
use adw::{ApplicationWindow, HeaderBar, NavigationPage, NavigationSplitView};
use gtk::{Box, Orientation, PolicyType, ScrolledWindow, Stack};

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
            .transition_type(gtk::StackTransitionType::Crossfade)
            .transition_duration(220)
            .build();

        stack.add_css_class("content-stack");

        let dashboard = Dashboard::new();

        let dashboard_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&dashboard.root)
            .build();

        let rgb_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&RgbPage::new())
            .build();
            
        let cooling_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&CoolingPage::new())
            .build();
            
        let performance_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&PerformancePage::new())
            .build();
            
        let battery_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&BatteryPage::new())
            .build();
            
        let settings_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Automatic)
            .hexpand(true)
            .vexpand(true)
            .child(&SettingsPage::new())
            .build();
            

        stack.add_titled(&dashboard_scroll, Some("dashboard"), "Dashboard");
        stack.add_titled(&rgb_scroll, Some("rgb"), "Keyboard RGB");
        stack.add_titled(&cooling_scroll, Some("cooling"), "Cooling");
        stack.add_titled(&performance_scroll, Some("performance"), "Performance");
        stack.add_titled(&battery_scroll, Some("battery"), "Battery");
        stack.add_titled(&settings_scroll, Some("settings"), "Settings");

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
            .show_title(false)
            .show_end_title_buttons(true)
            .build();

        header.add_css_class("main-headerbar");

        content.append(&header);
        content.append(&stack);

        let navigation = NavigationSplitView::builder()
            .sidebar(&sidebar_page)
            .content(
                &NavigationPage::builder()
                    .title("Acer Control Center")
                    .child(&content)
                    .build(),
            )
            .show_content(true)
            .collapsed(false)
            .build();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Acer Control Center")
            .default_width(1550)
            .default_height(920)
            .width_request(1200)
            .height_request(760)
            .content(&navigation)
            .build();

        window.add_css_class("main-window");

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
