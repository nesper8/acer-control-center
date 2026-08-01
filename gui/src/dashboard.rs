use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use adw::Clamp;
use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Orientation};

use crate::libacercontrol::driver;
use crate::libacercontrol::telemetry;
use crate::widgets::gauge::Gauge;
use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

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

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        content.set_hexpand(true);
        content.set_vexpand(false);

        let hero = HeroCard::new(
            "ACER ALG AL15G-53",
            "Acer Control Center",
            "SYSTEM ONLINE",
        );

        let hero_connected = StatusChip::new("CONNECTED", StatusColor::Green);
        let hero_driver = StatusChip::new("DRIVER", StatusColor::Blue);
        let hero_live = StatusChip::new("LIVE", StatusColor::Green);

        hero.add_chip(&hero_connected);
        hero.add_chip(&hero_driver);
        hero.add_chip(&hero_live);

        content.append(hero.widget());

        let overview_header = SectionHeader::new("SYSTEM OVERVIEW");
        let overview_status = StatusChip::new("LIVE", StatusColor::Green);
        overview_header.set_status(&overview_status);
        content.append(overview_header.widget());

        let driver_version = driver::version().unwrap_or_else(|_| "Unknown".into());
        let interface_name = driver::interface().unwrap_or_else(|_| "Unknown".into());
        let abi_version = format!("{}.{}", driver::abi_major(), driver::abi_minor());
        let hardware_supported = driver::hardware_supported().unwrap_or(false);

        let hardware_value = if hardware_supported {
            "SUPPORTED"
        } else {
            "UNSUPPORTED"
        };

        let hardware_subtitle = if hardware_supported {
            "Matching Acer hardware detected"
        } else {
            "Hardware unavailable"
        };

        let metric_grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        metric_grid.set_hexpand(true);
        metric_grid.set_column_homogeneous(true);

        let driver_card = MetricCard::new("Driver", &driver_version, "Kernel bridge");
        let abi_card = MetricCard::new("ABI", &abi_version, "Protocol version");
        let interface_card = MetricCard::new("Interface", &interface_name, "Active backend");
        let hardware_card = MetricCard::new("Hardware", hardware_value, hardware_subtitle);

        metric_grid.attach(driver_card.widget(), 0, 0, 1, 1);
        metric_grid.attach(abi_card.widget(), 1, 0, 1, 1);
        metric_grid.attach(interface_card.widget(), 0, 1, 1, 1);
        metric_grid.attach(hardware_card.widget(), 1, 1, 1, 1);

        content.append(&metric_grid);

        let cooling_header = SectionHeader::new("COOLING");
        let cooling_status = StatusChip::new("MONITORING", StatusColor::Blue);
        cooling_header.set_status(&cooling_status);
        content.append(cooling_header.widget());

        let gauges = Grid::builder()
            .column_spacing(32)
            .row_spacing(0)
            .column_homogeneous(true)
            .hexpand(true)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        let cpu_gauge = Rc::new(Gauge::new("CPU FAN"));
        let gpu_gauge = Rc::new(Gauge::new("GPU FAN"));

        cpu_gauge.set_color(0.22, 0.75, 1.0);
        gpu_gauge.set_color(1.0, 0.45, 0.16);

        gauges.attach(cpu_gauge.widget(), 0, 0, 1, 1);
        gauges.attach(gpu_gauge.widget(), 1, 0, 1, 1);

        let gauge_padding = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .hexpand(true)
            .halign(Align::Center)
            .valign(Align::Start)
            .build();

        gauge_padding.append(&gauges);

        let gauge_card = Frame::builder().build();
        gauge_card.set_child(Some(&gauge_padding));
        gauge_card.set_hexpand(true);
        gauge_card.set_halign(Align::Fill);
        gauge_card.set_valign(Align::Start);

        content.append(&gauge_card);

        root.append(&content);

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
