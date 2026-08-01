use adw::prelude::*;
use adw::Clamp;
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Label, Orientation};

use crate::libacercontrol::driver;
use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

pub struct SettingsPage;

impl SettingsPage {
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
            .maximum_size(1120)
            .tightening_threshold(960)
            .halign(Align::Center)
            .build();

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .build();

        let hero = HeroCard::new(
            "SETTINGS",
            "Application configuration and system information",
            "v0.1.0",
        );

        hero.add_chip(&StatusChip::new("DRIVER", StatusColor::Green));
        hero.add_chip(&StatusChip::new("DAEMON", StatusColor::Green));
        hero.add_chip(&StatusChip::new("GUI", StatusColor::Blue));

        content.append(hero.widget());

        let info_header = SectionHeader::new("SYSTEM INFORMATION");
        info_header.set_status(&StatusChip::new("LIVE", StatusColor::Green));

        content.append(info_header.widget());

        let driver_version = driver::version().unwrap_or_else(|_| "Unknown".into());

        let hardware = if driver::hardware_supported().unwrap_or(false) {
            "SUPPORTED"
        } else {
            "UNSUPPORTED"
        };

        let abi = format!("{}.{}", driver::abi_major(), driver::abi_minor());

        let interface = driver::interface().unwrap_or_else(|_| "Unknown".into());

        let grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .column_homogeneous(true)
            .build();

        grid.attach(
            MetricCard::new(
                "Driver Version",
                &driver_version,
                "Kernel driver",
            )
            .widget(),
            0,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Driver ABI",
                &abi,
                "Compatibility",
            )
            .widget(),
            1,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Interface",
                &interface,
                "Communication",
            )
            .widget(),
            0,
            1,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Hardware",
                hardware,
                "Detection",
            )
            .widget(),
            1,
            1,
            1,
            1,
        );

        content.append(&grid);

        let roadmap = SectionHeader::new("APPLICATION SETTINGS");
        roadmap.set_status(&StatusChip::new("COMING SOON", StatusColor::Orange));

        content.append(roadmap.widget());

        let frame = Frame::builder().build();

        let box_ = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        box_.append(
            &Label::builder()
                .label(
"Upcoming settings

• Launch at login

• Automatic update checks

• Theme selection

• Notification preferences

• Logging level

• Export diagnostics

• About Acer Control Center",
                )
                .wrap(true)
                .halign(Align::Start)
                .build(),
        );

        frame.set_child(Some(&box_));

        content.append(&frame);

        clamp.set_child(Some(&content));

        root.append(&clamp);

        root
    }
}
