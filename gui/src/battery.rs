use adw::prelude::*;
use adw::Clamp;
use gtk::prelude::*;
use gtk::{Align, Box, Frame, Grid, Label, Orientation};

use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

pub struct BatteryPage;

impl BatteryPage {
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
            "BATTERY",
            "Battery monitoring and charging management",
            "COMING SOON",
        );

        hero.add_chip(&StatusChip::new("POWER", StatusColor::Green));
        hero.add_chip(&StatusChip::new("MONITORING", StatusColor::Blue));
        hero.add_chip(&StatusChip::new("ROADMAP", StatusColor::Orange));

        content.append(hero.widget());

        let status = SectionHeader::new("CURRENT STATUS");
        status.set_status(&StatusChip::new("INFORMATION", StatusColor::Blue));

        content.append(status.widget());

        let grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .column_homogeneous(true)
            .build();

        grid.attach(
            MetricCard::new(
                "Charge Limit",
                "PLANNED",
                "Future feature",
            )
            .widget(),
            0,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Conservation",
                "PLANNED",
                "Future feature",
            )
            .widget(),
            1,
            0,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Battery Health",
                "PLANNED",
                "Future feature",
            )
            .widget(),
            0,
            1,
            1,
            1,
        );

        grid.attach(
            MetricCard::new(
                "Calibration",
                "PLANNED",
                "Future feature",
            )
            .widget(),
            1,
            1,
            1,
            1,
        );

        content.append(&grid);

        let roadmap = SectionHeader::new("UPCOMING FEATURES");
        roadmap.set_status(&StatusChip::new("ROADMAP", StatusColor::Orange));

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

        let label = Label::builder()
            .halign(Align::Start)
            .wrap(true)
            .label(
"• Battery Charge Limit

• Battery Conservation Mode

• Battery Health Information

• Battery Calibration

• Charging Behaviour

• Battery Wear Statistics

• Charge Cycle Counter

• Smart Charging Profiles",
            )
            .build();

        box_.append(&label);

        frame.set_child(Some(&box_));

        content.append(&frame);

        clamp.set_child(Some(&content));
        root.append(&clamp);

        root
    }
}
