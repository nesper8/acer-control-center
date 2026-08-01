use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use adw::Clamp;
use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Label, Orientation};

use crate::libacercontrol::driver;
use crate::libacercontrol::telemetry;
use crate::widgets::gauge::Gauge;

pub struct Dashboard {
    pub root: Box,
}

impl Dashboard {
    pub fn new() -> Self {
        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        root.set_hexpand(true);
        root.set_vexpand(false);

        let title = Label::builder()
            .label("Dashboard")
            .halign(Align::Start)
            .css_classes(["title-1"])
            .build();

        let info_card = Frame::builder().build();
        info_card.set_halign(Align::Fill);
        info_card.set_hexpand(true);
        info_card.set_vexpand(false);

        let info_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        let driver_label = Label::builder().halign(Align::Start).build();
        let abi_label = Label::builder().halign(Align::Start).build();
        let interface_label = Label::builder().halign(Align::Start).build();
        let hardware_label = Label::builder().halign(Align::Start).build();

        driver_label.set_label(&format!(
            "Driver Version : {}",
            driver::version().unwrap_or_else(|_| "Unknown".into())
        ));

        abi_label.set_label(&format!(
            "ABI            : {}.{}",
            driver::abi_major(),
            driver::abi_minor()
        ));

        interface_label.set_label(&format!(
            "Interface      : {}",
            driver::interface().unwrap_or_else(|_| "Unknown".into())
        ));

        hardware_label.set_label(&format!(
            "Hardware       : {}",
            if driver::hardware_supported().unwrap_or(false) {
                "Supported"
            } else {
                "Unsupported"
            }
        ));

        info_box.append(&driver_label);
        info_box.append(&abi_label);
        info_box.append(&interface_label);
        info_box.append(&hardware_label);

        info_card.set_child(Some(&info_box));

        root.append(&title);
        root.append(&info_card);

        let gauges = Grid::builder()
            .column_spacing(24)
            .row_spacing(0)
            .halign(Align::Center)
            .valign(Align::Start)
            .hexpand(false)
            .vexpand(false)
            .build();

        let cpu_gauge = Rc::new(Gauge::new("CPU Fan"));
        let gpu_gauge = Rc::new(Gauge::new("GPU Fan"));
        
        cpu_gauge.set_color(0.22, 0.75, 1.0);
        gpu_gauge.set_color(0.22, 0.75, 1.0);

        gauges.attach(cpu_gauge.widget(), 0, 0, 1, 1);
        gauges.attach(gpu_gauge.widget(), 1, 0, 1, 1);

        let gauge_padding = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .halign(Align::Center)
            .valign(Align::Start)
            .build();

        gauge_padding.append(&gauges);

        let gauge_card = Frame::builder().build();
        gauge_card.set_child(Some(&gauge_padding));
        gauge_card.set_halign(Align::Center);
        gauge_card.set_valign(Align::Start);
        gauge_card.set_hexpand(false);
        gauge_card.set_vexpand(false);

        root.append(&gauge_card);

        {
            let cpu = cpu_gauge.clone();
            let gpu = gpu_gauge.clone();

            glib::timeout_add_local(Duration::from_millis(500), move || {
                if let Ok(value) = telemetry::cpu_fan_rpm() {
                    cpu.set_value(value as f64);
                }

                if let Ok(value) = telemetry::gpu_fan_rpm() {
                    gpu.set_value(value as f64);
                }

                ControlFlow::Continue
            });
        }

        Self { root }
    }
}
