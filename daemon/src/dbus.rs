use std::sync::Arc;

use zbus::interface;

use crate::service::AcerService;

pub struct AcerControlDBus {
    service: Arc<AcerService>,
}

impl AcerControlDBus {
    pub fn new(service: Arc<AcerService>) -> Self {
        Self { service }
    }
}

#[interface(name = "io.github.stanlysilas.AcerControl")]
impl AcerControlDBus {
    async fn ping(&self) -> String {
        "pong".into()
    }

    async fn get_keyboard_brightness(
        &self,
    ) -> zbus::fdo::Result<u8> {
        self.service
            .get_keyboard_brightness()
            .map_err(zbus::fdo::Error::Failed)
    }

    async fn set_keyboard_brightness(
        &self,
        value: u8,
    ) -> zbus::fdo::Result<()> {
        self.service
            .set_keyboard_brightness(value)
            .map_err(zbus::fdo::Error::Failed)
    }

    async fn get_keyboard_color(
        &self,
    ) -> zbus::fdo::Result<(u8, u8, u8)> {
        self.service
            .get_keyboard_color()
            .map_err(zbus::fdo::Error::Failed)
    }

    async fn set_keyboard_color(
        &self,
        r: u8,
        g: u8,
        b: u8,
    ) -> zbus::fdo::Result<()> {
        self.service
            .set_keyboard_color(r, g, b)
            .map_err(zbus::fdo::Error::Failed)
    }
}
