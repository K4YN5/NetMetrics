use std::{io::Write, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::logs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(serialize_with = "serialize_tz", deserialize_with = "deserialize_tz")]
    pub timezone: chrono_tz::Tz,
    pub twelve_hour_format: bool,
    /// Interval in minutes
    pub interval: u16,
}

fn serialize_tz<S>(tz: &chrono_tz::Tz, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(tz.name())
}

fn deserialize_tz<'de, D>(deserializer: D) -> Result<chrono_tz::Tz, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let tz = String::deserialize(deserializer)?;
    chrono_tz::Tz::from_str(&tz).map_err(serde::de::Error::custom)
}

impl Config {
    /// Read the configuration from the config file, if it exists, or create it with the default
    pub fn initialize() -> Self {
        logs("Initializing configuration.");

        let mut default = Self::default();

        if let Ok(config) = std::fs::read_to_string("config") {
            logs("Configuration file found.");
            default.read_config(&config);
        } else {
            logs("Configuration file not found. Creating default configuration.");
            default.save();
        }

        default
    }

    fn read_config(&mut self, config_file: &str) {
        logs("Reading configuration from file.");
        *self = serde_json::from_str(config_file).unwrap_or({
            logs("Failed to read configuration from file. Resetting to default.");
            let default = Self::default();
            default.save();
            default
        });
    }

    /// Save the configuration to the config file, should be called after any changes. Idk how to
    /// enforce it yet.
    pub fn save(&self) {
        logs("Saving configuration to file.");
        let mut file = std::fs::File::create("config").unwrap();
        let config = serde_json::to_string(self).unwrap();
        file.write_all(config.as_bytes()).unwrap();
    }

    /// Mainly for debugging purposes, reset the configuration to the default values.
    pub fn reset(&mut self) {
        logs("Resetting configuration to default.");
        *self = Self::default();
        self.save();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timezone: chrono_tz::Tz::from_str("UTC").unwrap(),
            twelve_hour_format: false,
            interval: 5,
        }
    }
}
