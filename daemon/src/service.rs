use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::backend::HardwareBackend;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct KeyboardRgbSettings {
    brightness: u8,
    red: u8,
    green: u8,
    blue: u8,
}

struct KeyboardRgbSettingsStore {
    path: PathBuf,
}

impl KeyboardRgbSettingsStore {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load(&self) -> Result<Option<KeyboardRgbSettings>, String> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(error) => {
                return Err(format!(
                    "Failed to read keyboard RGB settings at {}: {error}",
                    self.path.display()
                ));
            }
        };

        let values: Result<Vec<u8>, _> =
            contents.split_whitespace().map(str::parse).collect();
        let values = values.map_err(|error| {
            format!(
                "Invalid keyboard RGB settings at {}: {error}",
                self.path.display()
            )
        })?;

        match values.as_slice() {
            [brightness, red, green, blue] => Ok(Some(KeyboardRgbSettings {
                brightness: *brightness,
                red: *red,
                green: *green,
                blue: *blue,
            })),
            _ => Err(format!(
                "Invalid keyboard RGB settings at {}: expected brightness and RGB values",
                self.path.display()
            )),
        }
    }

    fn save(&self, settings: KeyboardRgbSettings) -> Result<(), String> {
        let parent = self.path.parent().ok_or_else(|| {
            format!(
                "Keyboard RGB settings path has no parent: {}",
                self.path.display()
            )
        })?;
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Failed to create keyboard RGB settings directory {}: {error}",
                parent.display()
            )
        })?;

        let temporary_path = self.path.with_extension("tmp");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary_path)
            .map_err(|error| {
                format!(
                    "Failed to write keyboard RGB settings at {}: {error}",
                    temporary_path.display()
                )
            })?;
        writeln!(
            file,
            "{} {} {} {}",
            settings.brightness, settings.red, settings.green, settings.blue
        )
        .map_err(|error| {
            format!(
                "Failed to write keyboard RGB settings at {}: {error}",
                temporary_path.display()
            )
        })?;
        file.sync_all().map_err(|error| {
            format!(
                "Failed to sync keyboard RGB settings at {}: {error}",
                temporary_path.display()
            )
        })?;
        fs::rename(&temporary_path, &self.path).map_err(|error| {
            format!(
                "Failed to save keyboard RGB settings to {}: {error}",
                self.path.display()
            )
        })?;

        File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| {
                format!(
                    "Failed to sync keyboard RGB settings directory {}: {error}",
                    parent.display()
                )
            })
    }
}

pub struct AcerService {
    backend: Arc<dyn HardwareBackend>,
    rgb_settings: Mutex<KeyboardRgbSettings>,
    settings_store: KeyboardRgbSettingsStore,
}

impl AcerService {
    pub fn new(backend: Arc<dyn HardwareBackend>) -> Result<Self, String> {
        Self::with_settings_path(backend, "/var/lib/acer-control-daemon/keyboard-rgb")
    }

    fn with_settings_path(
        backend: Arc<dyn HardwareBackend>,
        settings_path: impl Into<PathBuf>,
    ) -> Result<Self, String> {
        let settings_store = KeyboardRgbSettingsStore::new(settings_path);
        let settings = match settings_store.load()? {
            Some(settings) => {
                backend.set_keyboard_color(settings.red, settings.green, settings.blue)?;
                backend.set_keyboard_brightness(settings.brightness)?;
                settings
            }
            None => {
                let brightness = backend.get_keyboard_brightness()?;
                let (red, green, blue) = backend.get_keyboard_color()?;
                let settings = KeyboardRgbSettings {
                    brightness,
                    red,
                    green,
                    blue,
                };
                settings_store.save(settings)?;
                settings
            }
        };

        Ok(Self {
            backend,
            rgb_settings: Mutex::new(settings),
            settings_store,
        })
    }

    pub fn get_keyboard_brightness(&self) -> Result<u8, String> {
        self.backend.get_keyboard_brightness()
    }

    pub fn set_keyboard_brightness(&self, value: u8) -> Result<(), String> {
        let mut settings = self
            .rgb_settings
            .lock()
            .map_err(|_| "Keyboard RGB settings lock poisoned".to_string())?;
        self.backend.set_keyboard_brightness(value)?;
        settings.brightness = value;
        self.settings_store.save(*settings)
    }

    pub fn get_keyboard_color(&self) -> Result<(u8, u8, u8), String> {
        self.backend.get_keyboard_color()
    }

    pub fn set_keyboard_color(&self, r: u8, g: u8, b: u8) -> Result<(), String> {
        let mut settings = self
            .rgb_settings
            .lock()
            .map_err(|_| "Keyboard RGB settings lock poisoned".to_string())?;
        self.backend.set_keyboard_color(r, g, b)?;
        settings.red = r;
        settings.green = g;
        settings.blue = b;
        self.settings_store.save(*settings)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AcerService, KeyboardRgbSettings, KeyboardRgbSettingsStore,
    };
    use crate::backend::HardwareBackend;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir()
            .join(format!("acer-control-settings-{}-{unique}", std::process::id()))
            .join("keyboard-rgb")
    }

    #[derive(Default)]
    struct TestBackend {
        brightness: Mutex<u8>,
        color: Mutex<(u8, u8, u8)>,
    }

    impl HardwareBackend for TestBackend {
        fn get_keyboard_brightness(&self) -> Result<u8, String> {
            Ok(*self.brightness.lock().unwrap())
        }

        fn set_keyboard_brightness(&self, value: u8) -> Result<(), String> {
            *self.brightness.lock().unwrap() = value;
            Ok(())
        }

        fn get_keyboard_color(&self) -> Result<(u8, u8, u8), String> {
            Ok(*self.color.lock().unwrap())
        }

        fn set_keyboard_color(
            &self,
            red: u8,
            green: u8,
            blue: u8,
        ) -> Result<(), String> {
            *self.color.lock().unwrap() = (red, green, blue);
            Ok(())
        }
    }

    #[test]
    fn settings_store_round_trips_values() {
        let path = test_path();
        let store = KeyboardRgbSettingsStore::new(&path);
        let expected = KeyboardRgbSettings {
            brightness: 123,
            red: 12,
            green: 34,
            blue: 56,
        };

        store.save(expected).unwrap();

        assert_eq!(store.load().unwrap(), Some(expected));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn settings_store_reports_invalid_values() {
        let path = test_path();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "256 0 0 0").unwrap();
        let store = KeyboardRgbSettingsStore::new(&path);

        assert!(store.load().is_err());
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn service_restores_saved_color_and_brightness() {
        let path = test_path();
        let first_backend = Arc::new(TestBackend::default());
        *first_backend.brightness.lock().unwrap() = 255;
        *first_backend.color.lock().unwrap() = (255, 255, 255);
        let service =
            AcerService::with_settings_path(first_backend, &path).unwrap();

        service.set_keyboard_brightness(87).unwrap();
        service.set_keyboard_color(10, 20, 30).unwrap();
        drop(service);

        let second_backend = Arc::new(TestBackend::default());
        let service =
            AcerService::with_settings_path(second_backend.clone(), &path).unwrap();

        assert_eq!(service.get_keyboard_brightness().unwrap(), 87);
        assert_eq!(service.get_keyboard_color().unwrap(), (10, 20, 30));
        fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
}
