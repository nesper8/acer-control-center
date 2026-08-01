use std::cell::Cell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::time::Duration;

use gtk::glib::{self, ControlFlow};
use gtk::prelude::*;
use gtk::{Align, Box, DrawingArea, Label, Orientation, Overlay};

struct GaugeState {
    current: Cell<f64>,
    target: Cell<f64>,
    color: Cell<(f64, f64, f64)>,
}

pub struct Gauge {
    root: Box,
    state: Rc<GaugeState>,
    drawing: DrawingArea,
    value_label: Label,
}

impl Gauge {
    pub fn new(title: &str) -> Self {
        let state = Rc::new(GaugeState {
            current: Cell::new(0.0_f64),
            target: Cell::new(0.0_f64),
            color: Cell::new((0.22, 0.75, 1.0)),
        });

        let root = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Center)
            .valign(Align::Start)
            .build();

        root.set_size_request(260, 220);
        root.set_hexpand(false);
        root.set_vexpand(false);

        let drawing = DrawingArea::builder()
            .content_width(260)
            .content_height(220)
            .build();

        drawing.set_size_request(260, 220);
        drawing.set_hexpand(false);
        drawing.set_vexpand(false);

        let value_label = Label::new(Some("0%"));
        value_label.add_css_class("title-1");
        value_label.set_halign(Align::Center);

        let name_label = Label::new(Some(title));
        name_label.add_css_class("heading");
        name_label.set_halign(Align::Center);

        let center_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(0)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();

        center_box.set_margin_top(0);
        center_box.append(&value_label);
        center_box.append(&name_label);

        let overlay = Overlay::new();
        overlay.set_child(Some(&drawing));
        overlay.add_overlay(&center_box);
        overlay.set_halign(Align::Center);
        overlay.set_valign(Align::Start);
        overlay.set_hexpand(false);
        overlay.set_vexpand(false);
        overlay.set_size_request(240, 220);

        let draw_state = state.clone();
        drawing.set_draw_func(move |_, cr, width, height| {
            let width = width as f64;
            let height = height as f64;
            let size = width.min(height);
            let cx = width / 2.0;
            let cy = height * 0.73;
            let radius = size * 0.45;

            let start = deg_to_rad(-150.0);
            let end = deg_to_rad(-30.0);
            let percent = draw_state.current.get().clamp(0.0_f64, 100.0_f64);
            let current_angle = angle_for_percent(percent, start, end);

            cr.set_antialias(gtk::cairo::Antialias::Best);
            cr.set_line_cap(gtk::cairo::LineCap::Round);

            cr.set_source_rgba(0.18, 0.19, 0.21, 1.0);
            cr.set_line_width(14.0);
            cr.arc(cx, cy, radius, start, end);
            cr.stroke().unwrap();

            let (r, g, b) = draw_state.color.get();
            cr.set_source_rgba(r, g, b, 1.0);
            cr.set_line_width(14.0);
            cr.arc(cx, cy, radius, start, current_angle);
            cr.stroke().unwrap();

            for tick in 0..=20 {
                let t = tick as f64 * 5.0;
                let angle = angle_for_percent(t, start, end);
                let is_major = tick % 5 == 0;

                let outer = radius + if is_major { 14.0 } else { 9.0 };
                let inner = radius - if is_major { 15.0 } else { 9.0 };

                let x1 = cx + inner * angle.cos();
                let y1 = cy + inner * angle.sin();
                let x2 = cx + outer * angle.cos();
                let y2 = cy + outer * angle.sin();

                if is_major {
                    cr.set_source_rgba(0.88, 0.90, 0.92, 1.0);
                    cr.set_line_width(2.2);
                } else {
                    cr.set_source_rgba(0.65, 0.67, 0.70, 1.0);
                    cr.set_line_width(1.2);
                }

                cr.move_to(x1, y1);
                cr.line_to(x2, y2);
                cr.stroke().unwrap();
            }

            let major_marks = [0.0_f64, 25.0, 50.0, 75.0, 100.0];
            cr.select_font_face(
                "Sans",
                gtk::cairo::FontSlant::Normal,
                gtk::cairo::FontWeight::Bold,
            );
            cr.set_font_size(size * 0.06);

            for mark in major_marks {
                let angle = angle_for_percent(mark, start, end);
                let label_radius = radius + 28.0;
                let x = cx + label_radius * angle.cos();
                let y = cy + label_radius * angle.sin();
                let text = format!("{:.0}", mark);

                cr.set_source_rgba(0.92, 0.92, 0.94, 1.0);
                cr.move_to(x - 10.0, y + 5.0);
                cr.show_text(&text).unwrap();
            }

            let needle_len = radius - 18.0;
            let needle_x = cx + needle_len * current_angle.cos();
            let needle_y = cy + needle_len * current_angle.sin();

            cr.set_source_rgba(0.95, 0.96, 0.98, 1.0);
            cr.set_line_width(3.5);
            cr.move_to(cx, cy);
            cr.line_to(needle_x, needle_y);
            cr.stroke().unwrap();

            cr.set_source_rgba(0.95, 0.96, 0.98, 1.0);
            cr.arc(cx, cy, 9.0, 0.0, PI * 2.0);
            cr.fill().unwrap();

            cr.set_source_rgba(0.10, 0.11, 0.13, 1.0);
            cr.arc(cx, cy, 4.0, 0.0, PI * 2.0);
            cr.fill().unwrap();
        });

        root.append(&overlay);

        let drawing_weak = drawing.downgrade();
        let value_label_weak = value_label.downgrade();
        let state_for_tick = state.clone();

        glib::timeout_add_local(Duration::from_millis(16), move || {
            let Some(drawing) = drawing_weak.upgrade() else {
                return ControlFlow::Break;
            };
            let Some(value_label) = value_label_weak.upgrade() else {
                return ControlFlow::Break;
            };

            let current = state_for_tick.current.get();
            let target = state_for_tick.target.get();
            let delta = target - current;

            let next = if delta.abs() < 0.05 {
                target
            } else {
                current + delta * 0.16
            };

            state_for_tick.current.set(next);

            value_label.set_label(&format!("{:.0}%", next.round()));
            drawing.queue_draw();

            ControlFlow::Continue
        });

        Self {
            root,
            state,
            drawing,
            value_label,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.root
    }

    pub fn set_value(&self, percent: f64) {
        self.state.target.set(percent.clamp(0.0_f64, 100.0_f64));
        self.drawing.queue_draw();
    }
    
    pub fn set_color(&self, r: f64, g: f64, b: f64) {
        self.state.color.set((r, g, b));
        self.drawing.queue_draw();
    }
}

fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn angle_for_percent(percent: f64, start: f64, end: f64) -> f64 {
    start + (end - start) * (percent / 100.0)
}
