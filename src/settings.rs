use anyhow::{Context, Result};
use chrono::{offset::Local, prelude::*};
use config::{Config, File};
use io::Write;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::{fs, io};
mod frequency;

// Logger configuration.
#[derive(Deserialize, Debug)]
pub struct Settings {
    pub wallabag: Wallabag,
    // path to timestamp file
    timestamp_path: PathBuf,
    // path to CoDual folder
    codual_path: String,
    // logging frequency
    frequency: frequency::Frequency,
}

impl Settings {
    pub fn from_file(path: &str) -> Result<Settings> {
        Config::builder()
            .add_source(File::with_name(path))
            .build()
            .context(format!("Failed to build config from {path}"))?
            .try_deserialize()
            .context(format!("Failed to deserialize config from {path}"))
    }

    pub fn ts(&self) -> Result<i64> {
        let s = fs::read_to_string(self.timestamp_path.clone()).context(format!(
            "Failed to read timestamp from {0}",
            self.timestamp_path.display()
        ))?;
        let ts = s.parse().context(format!(
            "Failed to parse timestamp from {0}",
            self.timestamp_path.display()
        ))?;
        Ok(ts)
    }

    fn set_ts(&self, ts: i64) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.timestamp_path)
            .context(format!(
                "Failed to update timestamp in {0}",
                self.timestamp_path.display()
            ))?;
        file.write_all(format!("{}", ts).as_bytes())?;
        Ok(())
    }

    pub fn update_ts(&self) -> Result<()> {
        self.set_ts(Local::now().timestamp())
    }

    pub fn current_path(&self) -> PathBuf {
        let today = Local::now();

        [
            &self.codual_path,
            "_log",
            &format!("{}", today.year()),
            &frequency::get_filename(&self.frequency, today),
        ]
        .iter()
        .collect()
    }
}

// Wallabag API secrets.
#[derive(Deserialize, Debug, Clone)]
pub struct Wallabag {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub base_url: String,
}

impl From<Wallabag> for wallabag_api::types::Config {
    fn from(secrets: Wallabag) -> Self {
        wallabag_api::types::Config {
            client_id: secrets.client_id,
            client_secret: secrets.client_secret,
            username: secrets.username,
            password: secrets.password,
            base_url: secrets.base_url,
        }
    }
}
