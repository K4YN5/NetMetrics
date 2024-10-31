use std::str::FromStr;

use chrono::DateTime;

#[derive(Debug)]
pub struct Date {
    /// Only the last 26 bits are used
    timestamp: u32,
}

impl Date {
    pub fn new() -> Self {
        let timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 60) as u32;

        Self { timestamp }
    }

    pub fn change_timezone(&mut self, timezone: &str) {
        let timezone = chrono_tz::Tz::from_str(timezone).unwrap();

        let mut config = crate::CONFIG.lock().unwrap();
        config.timezone = timezone;
        config.save();
    }

    pub fn change_time_format(&mut self, twelve_hour_format: bool) {
        let mut config = crate::CONFIG.lock().unwrap();
        config.twelve_hour_format = twelve_hour_format;
        config.save();
    }
}

// Implement the Display for the date to show as dd/mm/yyyy hh:mm using the chrono crate
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let datetime = DateTime::from_timestamp(self.timestamp as i64 * 60, 0).unwrap();
        let config = crate::CONFIG.lock().unwrap();

        let timezoned_date = datetime.with_timezone(&config.timezone);

        let format = if config.twelve_hour_format {
            "%d/%m/%Y %I:%M %p"
        } else {
            "%d/%m/%Y %H:%M"
        };

        // Format to `dd/mm/yyyy HH:MM`
        write!(f, "{}", timezoned_date.format(format))
    }
}
