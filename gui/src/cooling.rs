use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use adw::Clamp;
use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Orientation};

use crate::libacercontrol::telemetry;
use crate::widgets::gauge::Gauge;
use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

pub struct CoolingPage;

impl CoolingPage {
    pub fn new() -> Box {
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
        root.set_vexpand(true);

        let clamp = Clamp::builder()
            .maximum_size(1600)
            .tightening_threshold(1600)
            .halign(Align::Center)
            .valign(Align::Start)
            .build();

        clamp.set_hexpand(true);
        clamp.set_vexpand(true);

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        content.set_hexpand(true);
        content.set_vexpand(true);

        let hero = Rc::new(HeroCard::new(
            "COOLING SYSTEM",
            "Live fan percentage monitoring",
            "MONITORING",
        ));

        hero.add_chip(&StatusChip::new("LIVE", StatusColor::Green));
        hero.add_chip(&StatusChip::new("SYNCED", StatusColor::Blue));
        hero.add_chip(&StatusChip::new("AUTOMATIC", StatusColor::Orange));

        content.append(hero.widget());

        let overview_header = SectionHeader::new("COOLING OVERVIEW");
        overview_header.set_status(&StatusChip::new("LIVE", StatusColor::Green));
        content.append(overview_header.widget());

        let cpu_metric = Rc::new(MetricCard::new(
            "CPU Fan",
            "-- %",
            "Current percentage",
        ));
        let gpu_metric = Rc::new(MetricCard::new(
            "GPU Fan",
            "-- %",
            "Current percentage",
        ));
        let mode_metric = Rc::new(MetricCard::new(
            "Mode",
            "AUTOMATIC",
            "Daemon-managed",
        ));
        let daemon_metric = Rc::new(MetricCard::new(
            "Daemon",
            "RUNNING",
            "System service",
        ));

        let metrics = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        metrics.set_hexpand(true);
        metrics.set_column_homogeneous(true);

        metrics.attach(cpu_metric.widget(), 0, 0, 1, 1);
        metrics.attach(gpu_metric.widget(), 1, 0, 1, 1);
        metrics.attach(mode_metric.widget(), 0, 1, 1, 1);
        metrics.attach(daemon_metric.widget(), 1, 1, 1, 1);

        content.append(&metrics);

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

        let gauge_frame = Frame::builder().build();
        gauge_frame.set_child(Some(&gauge_padding));
        gauge_frame.set_hexpand(true);
        gauge_frame.set_halign(Align::Fill);
        gauge_frame.set_valign(Align::Start);

        content.append(&gauge_frame);

        let control_header = SectionHeader::new("MANUAL CONTROL");
        control_header.set_status(&StatusChip::new("COMING SOON", StatusColor::Orange));
        content.append(control_header.widget());

        let placeholder = Frame::builder().build();
        placeholder.set_hexpand(true);
        placeholder.set_halign(Align::Fill);
        placeholder.set_vexpand(false);

        let placeholder_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(8)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        let placeholder_label = gtk::Label::builder()
            .label("Manual fan control and custom fan curves will be added later.")
            .wrap(true)
            .halign(Align::Start)
            .build();

        placeholder_box.append(&placeholder_label);
        placeholder.set_child(Some(&placeholder_box));

        content.append(&placeholder);

        clamp.set_child(Some(&content));
        root.append(&clamp);

        {
            let hero = hero.clone();
            let cpu_metric = cpu_metric.clone();
            let gpu_metric = gpu_metric.clone();
            let cpu_gauge = cpu_gauge.clone();
            let gpu_gauge = gpu_gauge.clone();

            glib::timeout_add_local(Duration::from_millis(500), move || {
                let mut cpu_value = None;
                let mut gpu_value = None;

                if let Ok(value) = telemetry::cpu_fan_rpm() {
                    cpu_value = Some(value);
                    cpu_metric.set_value(&format!("{} %", value));
                    cpu_gauge.set_value(value as f64);
                }

                if let Ok(value) = telemetry::gpu_fan_rpm() {
                    gpu_value = Some(value);
                    gpu_metric.set_value(&format!("{} %", value));
                    gpu_gauge.set_value(value as f64);
                }

                if let (Some(cpu), Some(gpu)) = (cpu_value, gpu_value) {
                    let avg = (cpu + gpu) / 2;
                    hero.set_value(&format!("{}% AVG", avg));
                }

                ControlFlow::Continue
            });
        }

        root
    }
}
