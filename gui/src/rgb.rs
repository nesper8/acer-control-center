use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use adw::Clamp;
use gtk::gdk::RGBA;
use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;
use gtk::{
    Align, Box, ColorDialog, ColorDialogButton, DrawingArea, Frame, Grid, Label, Orientation, Scale,
};

use crate::libacercontrol::dbus::AcerControlDBus;
use crate::widgets::hero_card::HeroCard;
use crate::widgets::metric_card::MetricCard;
use crate::widgets::section_header::SectionHeader;
use crate::widgets::status_chip::{StatusChip, StatusColor};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LedColor {
    r: u8,
    g: u8,
    b: u8,
}

struct RgbUiState {
    syncing: Cell<bool>,
    brightness: Cell<u8>,
    color: Cell<LedColor>,
}

pub struct RgbPage;

impl RgbPage {
    pub fn new() -> Box {
        let initial_brightness = read_brightness_from_daemon().unwrap_or(255);
        let initial_color = read_color_from_daemon().unwrap_or(LedColor {
            r: 255,
            g: 255,
            b: 255,
        });

        let state = Rc::new(RgbUiState {
            syncing: Cell::new(false),
            brightness: Cell::new(initial_brightness),
            color: Cell::new(initial_color),
        });

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

        let clamp = Clamp::builder()
            .maximum_size(1120)
            .tightening_threshold(960)
            .halign(Align::Center)
            .valign(Align::Start)
            .build();

        clamp.set_hexpand(true);
        clamp.set_vexpand(false);

        let content = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(18)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        content.set_hexpand(true);
        content.set_vexpand(false);

        let hero = Rc::new(HeroCard::new(
            "KEYBOARD RGB",
            "Acer keyboard lighting",
            &format_color_hex(initial_color),
        ));

        let hero_connected = StatusChip::new("CONNECTED", StatusColor::Green);
        let hero_driver = StatusChip::new("DAEMON", StatusColor::Blue);
        let hero_live = StatusChip::new("LIVE", StatusColor::Green);

        hero.add_chip(&hero_connected);
        hero.add_chip(&hero_driver);
        hero.add_chip(&hero_live);

        content.append(hero.widget());

        let overview_header = SectionHeader::new("SYSTEM OVERVIEW");
        let overview_status = StatusChip::new("SYNCED", StatusColor::Blue);
        overview_header.set_status(&overview_status);
        content.append(overview_header.widget());

        let brightness_metric = Rc::new(MetricCard::new(
            "Brightness",
            &format!("{}/255", initial_brightness),
            "LED intensity",
        ));
        let color_metric = Rc::new(MetricCard::new(
            "Color",
            &format_color_hex(initial_color),
            "RGB triplet",
        ));
        let backend_metric = MetricCard::new("Backend", "RUNNING", "System daemon");
        let device_metric = MetricCard::new("Device", "rgb:kbd_backlight", "LED node");

        let metric_grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        metric_grid.set_hexpand(true);
        metric_grid.set_column_homogeneous(true);

        metric_grid.attach(brightness_metric.widget(), 0, 0, 1, 1);
        metric_grid.attach(color_metric.widget(), 1, 0, 1, 1);
        metric_grid.attach(backend_metric.widget(), 0, 1, 1, 1);
        metric_grid.attach(device_metric.widget(), 1, 1, 1, 1);

        content.append(&metric_grid);

        let lighting_header = SectionHeader::new("LIGHTING PANEL");
        let lighting_status = StatusChip::new("ACTIVE", StatusColor::Green);
        lighting_header.set_status(&lighting_status);
        content.append(lighting_header.widget());

        let split_grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        split_grid.set_hexpand(true);
        split_grid.set_column_homogeneous(true);

        let preview_card = Frame::builder().build();
        preview_card.add_css_class("rgb-panel-card");
        preview_card.add_css_class("rgb-preview-card");
        preview_card.set_hexpand(true);
        preview_card.set_halign(Align::Fill);
        preview_card.set_vexpand(false);

        let preview_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        let preview_label = Label::builder()
            .label("Keyboard Preview")
            .halign(Align::Start)
            .css_classes(["heading"])
            .build();

        let preview_area = DrawingArea::builder()
            .content_width(620)
            .content_height(220)
            .build();

        preview_area.set_size_request(620, 220);
        preview_area.set_hexpand(false);
        preview_area.set_vexpand(false);

        {
            let draw_state = state.clone();
            preview_area.set_draw_func(move |_, cr, width, height| {
                let width = width as f64;
                let height = height as f64;
                let color = draw_state.color.get();
                let brightness = draw_state.brightness.get();
                draw_keyboard_preview(cr, width, height, color, brightness);
            });
        }

        preview_box.append(&preview_label);
        preview_box.append(&preview_area);
        preview_card.set_child(Some(&preview_box));

        let controls_card = Frame::builder().build();
        controls_card.add_css_class("rgb-panel-card");
        controls_card.add_css_class("rgb-controls-card");
        controls_card.set_hexpand(true);
        controls_card.set_halign(Align::Fill);
        controls_card.set_vexpand(false);

        let controls_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(14)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        let controls_label = Label::builder()
            .label("Controls")
            .halign(Align::Start)
            .css_classes(["rgb-control-heading"])
            .build();

        let controls_grid = Grid::builder()
            .column_spacing(16)
            .row_spacing(12)
            .halign(Align::Fill)
            .valign(Align::Start)
            .build();

        controls_grid.set_hexpand(true);

        let color_label = Label::builder()
            .label("Color")
            .halign(Align::Start)
            .build();

        let color_dialog = ColorDialog::builder().modal(true).build();

        let color_button = ColorDialogButton::builder().build();
        color_button.set_dialog(&color_dialog);
        color_button.set_rgba(&rgba_from_led_color(initial_color));

        let brightness_label = Label::builder()
            .label("Brightness")
            .halign(Align::Start)
            .build();

        let brightness_scale = Scale::with_range(Orientation::Horizontal, 0.0, 255.0, 1.0);
        brightness_scale.set_digits(0);
        brightness_scale.set_draw_value(true);
        brightness_scale.set_hexpand(true);
        brightness_scale.set_value(initial_brightness as f64);

        let readout_label = Label::builder()
            .halign(Align::Start)
            .wrap(true)
            .build();

        readout_label.add_css_class("rgb-readout");

        update_readout(&readout_label, initial_color, initial_brightness);

        controls_grid.attach(&color_label, 0, 0, 1, 1);
        controls_grid.attach(&color_button, 1, 0, 1, 1);
        controls_grid.attach(&brightness_label, 0, 1, 1, 1);
        controls_grid.attach(&brightness_scale, 1, 1, 1, 1);
        controls_grid.attach(&readout_label, 0, 2, 2, 1);

        controls_box.append(&controls_label);
        controls_box.append(&controls_grid);
        controls_card.set_child(Some(&controls_box));

        split_grid.attach(&preview_card, 0, 0, 1, 1);
        split_grid.attach(&controls_card, 1, 0, 1, 1);

        content.append(&split_grid);
        clamp.set_child(Some(&content));
        root.append(&clamp);

        refresh_display(
            &hero,
            &brightness_metric,
            &color_metric,
            &readout_label,
            initial_brightness,
            initial_color,
        );

        {
            let state = state.clone();
            let hero = hero.clone();
            let brightness_metric = brightness_metric.clone();
            let color_metric = color_metric.clone();
            let preview_area = preview_area.clone();
            let readout_label = readout_label.clone();

            color_button.connect_rgba_notify(move |button| {
                if state.syncing.get() {
                    return;
                }

                let color = led_color_from_rgba(&button.rgba());
                state.color.set(color);

                match write_color_via_daemon(color) {
                    Ok(()) => {
                        refresh_display(
                            &hero,
                            &brightness_metric,
                            &color_metric,
                            &readout_label,
                            state.brightness.get(),
                            color,
                        );
                        preview_area.queue_draw();
                    }
                    Err(err) => {
                        readout_label.set_label(&err);
                    }
                }
            });
        }

        {
            let state = state.clone();
            let hero = hero.clone();
            let brightness_metric = brightness_metric.clone();
            let color_metric = color_metric.clone();
            let preview_area = preview_area.clone();
            let readout_label = readout_label.clone();

            brightness_scale.connect_value_changed(move |scale| {
                if state.syncing.get() {
                    return;
                }

                let brightness = scale.value().round().clamp(0.0, 255.0) as u8;
                state.brightness.set(brightness);

                match write_brightness_via_daemon(brightness) {
                    Ok(()) => {
                        refresh_display(
                            &hero,
                            &brightness_metric,
                            &color_metric,
                            &readout_label,
                            brightness,
                            state.color.get(),
                        );
                        preview_area.queue_draw();
                    }
                    Err(err) => {
                        readout_label.set_label(&err);
                    }
                }
            });
        }

        {
            let state = state.clone();
            let hero = hero.clone();
            let brightness_metric = brightness_metric.clone();
            let color_metric = color_metric.clone();
            let preview_area = preview_area.clone();
            let color_button = color_button.clone();
            let brightness_scale = brightness_scale.clone();
            let readout_label = readout_label.clone();

            glib::timeout_add_local(Duration::from_millis(1000), move || {
                if state.syncing.get() {
                    return ControlFlow::Continue;
                }

                let mut changed = false;

                match read_brightness_from_daemon() {
                    Ok(brightness) => {
                        if brightness != state.brightness.get() {
                            state.syncing.set(true);
                            brightness_scale.set_value(brightness as f64);
                            state.brightness.set(brightness);
                            state.syncing.set(false);
                            changed = true;
                        }
                    }
                    Err(err) => {
                        readout_label.set_label(&err);
                    }
                }

                match read_color_from_daemon() {
                    Ok(color) => {
                        if color != state.color.get() {
                            state.syncing.set(true);
                            color_button.set_rgba(&rgba_from_led_color(color));
                            state.color.set(color);
                            state.syncing.set(false);
                            changed = true;
                        }
                    }
                    Err(err) => {
                        readout_label.set_label(&err);
                    }
                }

                if changed {
                    refresh_display(
                        &hero,
                        &brightness_metric,
                        &color_metric,
                        &readout_label,
                        state.brightness.get(),
                        state.color.get(),
                    );
                    preview_area.queue_draw();
                }

                ControlFlow::Continue
            });
        }

        root
    }
}

fn read_brightness_from_daemon() -> Result<u8, String> {
    AcerControlDBus::new()?.keyboard_brightness()
}

fn read_color_from_daemon() -> Result<LedColor, String> {
    let (r, g, b) = AcerControlDBus::new()?.keyboard_color()?;
    Ok(LedColor { r, g, b })
}

fn write_brightness_via_daemon(brightness: u8) -> Result<(), String> {
    AcerControlDBus::new()?.set_keyboard_brightness(brightness)
}

fn write_color_via_daemon(color: LedColor) -> Result<(), String> {
    AcerControlDBus::new()?.set_keyboard_color(color.r, color.g, color.b)
}

fn refresh_display(
    hero: &HeroCard,
    brightness_metric: &MetricCard,
    color_metric: &MetricCard,
    readout_label: &Label,
    brightness: u8,
    color: LedColor,
) {
    let hex = format_color_hex(color);

    hero.set_value(&hex);
    brightness_metric.set_value(&format!("{}/255", brightness));
    color_metric.set_value(&hex);
    update_readout(readout_label, color, brightness);
}

fn led_color_from_rgba(rgba: &RGBA) -> LedColor {
    LedColor {
        r: (rgba.red() * 255.0).round().clamp(0.0, 255.0) as u8,
        g: (rgba.green() * 255.0).round().clamp(0.0, 255.0) as u8,
        b: (rgba.blue() * 255.0).round().clamp(0.0, 255.0) as u8,
    }
}

fn rgba_from_led_color(color: LedColor) -> RGBA {
    RGBA::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        1.0,
    )
}

fn format_color_hex(color: LedColor) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}

fn update_readout(label: &Label, color: LedColor, brightness: u8) {
    label.set_label(&format!(
        "{}   ·   Brightness: {}/255   ·   RGB: {}, {}, {}",
        format_color_hex(color),
        brightness,
        color.r,
        color.g,
        color.b
    ));
}

fn draw_keyboard_preview(
    cr: &gtk::cairo::Context,
    width: f64,
    height: f64,
    color: LedColor,
    brightness: u8,
) {
    let factor = brightness as f64 / 255.0;
    let preview_r = color.r as f64 / 255.0 * factor;
    let preview_g = color.g as f64 / 255.0 * factor;
    let preview_b = color.b as f64 / 255.0 * factor;

    cr.set_antialias(gtk::cairo::Antialias::Best);
    cr.set_line_cap(gtk::cairo::LineCap::Round);
    cr.set_line_join(gtk::cairo::LineJoin::Round);

    cr.set_source_rgba(0.08, 0.09, 0.10, 1.0);
    rounded_rect(cr, 8.0, 8.0, width - 16.0, height - 16.0, 18.0);
    cr.fill().unwrap();

    cr.set_source_rgba(0.14, 0.15, 0.17, 1.0);
    rounded_rect(cr, 24.0, 28.0, width - 48.0, height - 56.0, 16.0);
    cr.fill().unwrap();

    cr.set_source_rgba(0.20, 0.21, 0.23, 1.0);
    rounded_rect(cr, 36.0, 40.0, width - 72.0, height - 84.0, 12.0);
    cr.fill().unwrap();

    let body_x = 42.0;
    let body_y = 48.0;
    let body_w = width - 84.0;
    let key_h = 26.0;
    let key_gap = 5.0;
    let row_gap = 11.0;

    draw_key_row(
        cr,
        body_x,
        body_y,
        body_w,
        13,
        0.0,
        key_h,
        key_gap,
        preview_r,
        preview_g,
        preview_b,
    );

    draw_key_row(
        cr,
        body_x + 18.0,
        body_y + key_h + row_gap,
        body_w - 36.0,
        12,
        0.0,
        key_h,
        key_gap,
        preview_r,
        preview_g,
        preview_b,
    );

    draw_key_row(
        cr,
        body_x + 36.0,
        body_y + (key_h + row_gap) * 2.0,
        body_w - 72.0,
        11,
        0.0,
        key_h,
        key_gap,
        preview_r,
        preview_g,
        preview_b,
    );

    let spacebar_y = body_y + (key_h + row_gap) * 3.0 + 6.0;
    let spacebar_x = body_x + 80.0;
    let spacebar_w = body_w - 160.0;
    cr.set_source_rgba(preview_r * 0.75, preview_g * 0.75, preview_b * 0.75, 1.0);
    rounded_rect(cr, spacebar_x, spacebar_y, spacebar_w, 24.0, 10.0);
    cr.fill_preserve().unwrap();
    cr.set_source_rgba(0.10, 0.11, 0.13, 1.0);
    cr.set_line_width(1.0);
    cr.stroke().unwrap();

    cr.set_source_rgba(0.60, 0.62, 0.66, 1.0);
    cr.select_font_face(
        "Sans",
        gtk::cairo::FontSlant::Normal,
        gtk::cairo::FontWeight::Bold,
    );
    cr.set_font_size(18.0);
    cr.move_to(44.0, 28.0);
}

fn draw_key_row(
    cr: &gtk::cairo::Context,
    start_x: f64,
    y: f64,
    total_width: f64,
    count: usize,
    row_shift: f64,
    key_h: f64,
    key_gap: f64,
    preview_r: f64,
    preview_g: f64,
    preview_b: f64,
) {
    let key_w = (total_width - (count as f64 - 1.0) * key_gap) / count as f64;
    let x_offset = start_x + row_shift;

    for index in 0..count {
        let x = x_offset + index as f64 * (key_w + key_gap);
        cr.set_source_rgba(preview_r, preview_g, preview_b, 1.0);
        rounded_rect(cr, x, y, key_w, key_h, 5.0);
        cr.fill_preserve().unwrap();
        cr.set_source_rgba(0.10, 0.11, 0.13, 1.0);
        cr.set_line_width(1.0);
        cr.stroke().unwrap();
    }
}

fn rounded_rect(cr: &gtk::cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, (-90.0_f64).to_radians(), 0.0_f64.to_radians());
    cr.arc(x + w - r, y + h - r, r, 0.0_f64.to_radians(), 90.0_f64.to_radians());
    cr.arc(
        x + r,
        y + h - r,
        r,
        90.0_f64.to_radians(),
        180.0_f64.to_radians(),
    );
    cr.arc(x + r, y + r, r, 180.0_f64.to_radians(), 270.0_f64.to_radians());
    cr.close_path();
}
