use zbus::blocking::{Connection, Proxy};

const SERVICE: &str = "io.github.stanlysilas.AcerControl";
const PATH: &str = "/io/github/stanlysilas/AcerControl";
const INTERFACE: &str = "io.github.stanlysilas.AcerControl";

pub struct AcerControlDBus {
    connection: Connection,
}

impl AcerControlDBus {
    pub fn new() -> Result<Self, String> {
        let connection = Connection::system()
            .map_err(|e| e.to_string())?;

        Ok(Self { connection })
    }

    fn proxy(&self) -> Result<Proxy<'_>, String> {
        Proxy::new(
            &self.connection,
            SERVICE,
            PATH,
            INTERFACE,
        )
        .map_err(|e| e.to_string())
    }

    pub fn ping(&self) -> Result<String, String> {
        let proxy = self.proxy()?;

        proxy
            .call("Ping", &())
            .map_err(|e| e.to_string())
    }

    pub fn keyboard_brightness(&self) -> Result<u8, String> {
        let proxy = self.proxy()?;

        proxy
            .call("GetKeyboardBrightness", &())
            .map_err(|e| e.to_string())
    }

    pub fn set_keyboard_brightness(&self, brightness: u8) -> Result<(), String> {
        let proxy = self.proxy()?;

        proxy
            .call::<_, _, ()>("SetKeyboardBrightness", &(brightness))
            .map_err(|e| e.to_string())
    }

    pub fn keyboard_color(&self) -> Result<(u8, u8, u8), String> {
        let proxy = self.proxy()?;

        proxy
            .call("GetKeyboardColor", &())
            .map_err(|e| e.to_string())
    }

    pub fn set_keyboard_color(
        &self,
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<(), String> {
        let proxy = self.proxy()?;

        proxy
            .call::<_, _, ()>(
                "SetKeyboardColor",
                &(red, green, blue),
            )
            .map_err(|e| e.to_string())
    }
}
