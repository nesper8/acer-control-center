use adw::prelude::*;
use gtk::{
    Align, Box, Label, ListBox, ListBoxRow, Orientation, SelectionMode, Stack,
};

pub struct Sidebar {
    container: Box,
}

impl Sidebar {
    pub fn new(stack: &Stack) -> Box {
        let list = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .css_classes(vec!["navigation-sidebar"])
            .build();

        add_row(&list, "Dashboard");
        add_row(&list, "RGB");
        add_row(&list, "Cooling");
        add_row(&list, "Performance");
        add_row(&list, "Battery");
        add_row(&list, "Settings");

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

        let container = Box::builder()
            .orientation(Orientation::Vertical)
            .width_request(240)
            .vexpand(true)
            .build();

        container.append(&list);

        container
    }
}

fn add_row(list: &ListBox, title: &str) {
    let label = Label::builder()
        .label(title)
        .halign(Align::Start)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(18)
        .margin_end(18)
        .build();

    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    row_box.append(&label);

    let row = ListBoxRow::new();
    row.set_child(Some(&row_box));

    list.append(&row);
}
