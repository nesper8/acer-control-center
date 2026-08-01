use adw::prelude::*;
use gtk::{
    Align, Box, Image, Label, ListBox, ListBoxRow, Orientation, SelectionMode, Stack,
};

pub struct Sidebar;

impl Sidebar {
    pub fn new(stack: &Stack) -> Box {
        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(280)
            .hexpand(false)
            .vexpand(true)
            .build();

        root.add_css_class("sidebar");

        let branding = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .margin_top(28)
            .margin_bottom(24)
            .margin_start(20)
            .margin_end(20)
            .build();

        let logo = Image::from_icon_name("computer-symbolic");
        logo.set_pixel_size(42);

        let title = Label::builder()
            .label("ACER")
            .halign(Align::Start)
            .build();

        title.add_css_class("sidebar-title");

        let subtitle = Label::builder()
            .label("CONTROL CENTER")
            .halign(Align::Start)
            .build();

        subtitle.add_css_class("sidebar-subtitle");

        branding.append(&logo);
        branding.append(&title);
        branding.append(&subtitle);

        let list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .build();

        list.add_css_class("sidebar-list");

        add_row(&list, "go-home-symbolic", "Dashboard");
        add_row(&list, "input-keyboard-symbolic", "Keyboard RGB");
        add_row(&list, "weather-windy-symbolic", "Cooling");
        add_row(&list, "speedometer-symbolic", "Performance");
        add_row(&list, "battery-good-symbolic", "Battery");
        add_row(&list, "preferences-system-symbolic", "Settings");

        let stack_clone = stack.clone();

        list.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                match row.index() {
                    0 => stack_clone.set_visible_child_name("dashboard"),
                    1 => stack_clone.set_visible_child_name("rgb"),
                    2 => stack_clone.set_visible_child_name("cooling"),
                    3 => stack_clone.set_visible_child_name("performance"),
                    4 => stack_clone.set_visible_child_name("battery"),
                    5 => stack_clone.set_visible_child_name("settings"),
                    _ => {}
                }
            }
        });

        list.select_row(list.row_at_index(0).as_ref());

        root.append(&branding);
        root.append(&list);

        root
    }
}

fn add_row(list: &ListBox, icon: &str, title: &str) {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(14)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(18)
        .margin_end(18)
        .build();

    let image = Image::from_icon_name(icon);
    image.set_pixel_size(20);

    let label = Label::builder()
        .label(title)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    label.add_css_class("sidebar-item-label");

    row_box.append(&image);
    row_box.append(&label);

    let row = ListBoxRow::new();
    row.add_css_class("sidebar-row");
    row.set_child(Some(&row_box));

    list.append(&row);
}
