use anyhow::Result;
use chrono::offset::Local;
use config::{Config, File};
use io::Write;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Deserialize)]
enum Frequency {
    Weekly,
    Monthly,
    Bimonthly,
}

impl Frequency {
    fn prefix(&self) -> &'static str {
        match self {
            Frequency::Weekly => "w",
            Frequency::Monthly => "m",
            Frequency::Bimonthly => "b",
        }
    }
}

// Logger configuration.
#[derive(Deserialize, Debug)]
pub struct Settings {
    pub wallabag: Wallabag,
    // path to timestamp file
    timestamp_path: PathBuf,
    // path to CoDual folder
    codual_path: String,
    // logging frequency
    frequency: Frequency,
}

impl Settings {
    pub fn from_file(path: &str) -> Result<Settings> {
        let mut settings = Config::default();
        settings.merge(File::with_name(path))?;
        let result = settings.try_into()?;
        Ok(result)
    }

    pub fn ts(&self) -> Result<i64> {
        let s = fs::read_to_string(self.timestamp_path.clone())?;
        let ts = s.parse()?;
        Ok(ts)
    }

    fn set_ts(&self, ts: i64) -> Result<()> {
        let mut file = OpenOptions::new().write(true).open(&self.timestamp_path)?;
        file.write_all(format!("{}", ts).as_bytes())?;
        Ok(())
    }

    pub fn update_ts(&self) -> Result<()> {
        self.set_ts(Local::now().timestamp())
    }

    pub fn current_path(&self) -> PathBuf {
        [
            &self.codual_path,
            "_log",
            "2021",
            &format!("{}{}", self.frequency.prefix(), 3),
            "output.md",
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
