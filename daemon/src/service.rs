use std::sync::Arc;

use crate::backend::HardwareBackend;

pub struct AcerService {
    backend: Arc<dyn HardwareBackend>,
}

impl AcerService {
    pub fn new(
        backend: Arc<dyn HardwareBackend>,
    ) -> Self {
        Self { backend }
    }

    pub fn get_keyboard_brightness(
        &self,
    ) -> Result<u8, String> {
        self.backend.get_keyboard_brightness()
    }

    pub fn set_keyboard_brightness(
        &self,
        value: u8,
    ) -> Result<(), String> {
        self.backend.set_keyboard_brightness(value)
    }

    pub fn get_keyboard_color(
        &self,
    ) -> Result<(u8, u8, u8), String> {
        self.backend.get_keyboard_color()
    }

    pub fn set_keyboard_color(
        &self,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), String> {
        self.backend.set_keyboard_color(r, g, b)
    }
}
