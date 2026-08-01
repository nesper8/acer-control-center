use gtk::prelude::*;
use gtk::{Align, Box, Label, Orientation};

pub struct SettingsPage;

impl SettingsPage {
    pub fn new() -> Box {
        let page = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .spacing(12)
            .hexpand(true)
            .vexpand(true)
            .build();

        page.append(
            &Label::builder()
                .label("Settings")
                .css_classes(vec!["title-1"])
                .halign(Align::Start)
                .build(),
        );

        page.append(
            &Label::builder()
                .label("Application settings will appear here.")
                .halign(Align::Start)
                .build(),
        );

        page
    }
}
