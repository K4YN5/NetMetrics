use std::{iter::Map, process::Command, str::Lines};

use crate::{date::Date, error, logs, FAIL_COUNT};

#[derive(Debug)]
pub struct Record {
    // Stored with a decimal of precision, so divide by ten to get teh value.
    pub download: u16, // Mbps
    pub upload: u16,   // Mbps
    pub ping: u16,     //Mbps

    pub timestamp: Date,
}

#[derive(Debug)]
pub enum RecordError {
    Fatal,
    /// The u8 is a timeout in seconds
    NonFatal(u8),
    TooBig,
}

impl Record {
    pub fn new() -> Result<Record, RecordError> {
        logs("Running speedtest-cli.");
        let command = unsafe {
            String::from_utf8_unchecked(
                Command::new("speedtest-cli")
                    .args(["--secure", "--simple"])
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

        logs("Parsing speedtest-cli output.");
        let mut result = command
            .lines()
            .map(|line| line.split_whitespace().nth(1).unwrap())
            .collect::<Vec<&str>>();

        let parser = |result: &mut Vec<&str>| -> Result<u16, RecordError> {
            if let Ok(decimal) = result.pop().unwrap().parse::<f32>() {
                *FAIL_COUNT.lock().unwrap() = 0;
                Ok(((decimal * 10.0) as u32).try_into().map_err(|_| {
                    error("\"Sadly\" the error was too big.");
                    RecordError::TooBig
                })?)
            } else {
                error("Failed to parse peedtest-cli output into a f32.");
                if *FAIL_COUNT.lock().unwrap() >= 3 {
                    error("Failed to parse speedtest-cli output 3 times in a row. Exiting.");
                    return Err(RecordError::Fatal);
                }
                return Err(RecordError::NonFatal(10));
            }
        };

        let upload = parser(&mut result)?;
        let download = parser(&mut result)?;
        let ping = parser(&mut result)?;

        let timestamp = Date::new();

        Ok(Record {
            download,
            upload,
            ping,
            timestamp,
        })
    }
}
