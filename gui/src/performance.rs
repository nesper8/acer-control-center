use adw::prelude::*;
use adw::Clamp;
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Label, Orientation};

use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

pub struct PerformancePage;

impl PerformancePage {
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
            "PERFORMANCE",
            "Future power and performance management",
            "COMING SOON",
        );

        hero.add_chip(&StatusChip::new("DRIVER READY", StatusColor::Green));
        hero.add_chip(&StatusChip::new("DAEMON READY", StatusColor::Green));
        hero.add_chip(&StatusChip::new("GUI READY", StatusColor::Blue));

        content.append(hero.widget());

        let status = SectionHeader::new("FEATURE STATUS");
        status.set_status(&StatusChip::new("ROADMAP", StatusColor::Orange));

        content.append(status.widget());

        let grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .column_homogeneous(true)
            .build();

        grid.attach(
            MetricCard::new("Daemon", "READY", "IPC implemented").widget(),
            0,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new("Driver", "READY", "Kernel module").widget(),
            1,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new("GUI", "READY", "Widgets complete").widget(),
            0,
            1,
            1,
            1,
        );

        grid.attach(
            MetricCard::new("Profiles", "PLANNED", "Next milestone").widget(),
            1,
            1,
            1,
            1,
        );

        content.append(&grid);

        let roadmap = SectionHeader::new("UPCOMING FEATURES");
        roadmap.set_status(&StatusChip::new("DEVELOPMENT", StatusColor::Blue));

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

        let text = Label::builder()
            .halign(Align::Start)
            .wrap(true)
            .label(
"• Balanced Mode

• Performance Mode

• Turbo Mode

• Eco Mode

• CPU Power Limits

• Platform Profiles

• Automatic Profile Switching

• Battery-aware Performance")
            .build();

        box_.append(&text);

        frame.set_child(Some(&box_));

        content.append(&frame);

        clamp.set_child(Some(&content));

        root.append(&clamp);

        root
    }
}
