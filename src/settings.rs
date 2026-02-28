use anyhow::{Context, Result};
use chrono::{offset::Local, prelude::*};
use config::{Config, File};
use serde::Deserialize;
use std::path::PathBuf;

use crate::karakeep;
mod frequency;

/// Logger configuration.
#[derive(Deserialize, Debug)]
pub struct Settings {
    pub karakeep: karakeep::Config,
    codual_path: String,
    frequency: frequency::Frequency,
}

impl Settings {
    pub fn from_file(path: &str) -> Result<Self> {
        Config::builder()
            .add_source(File::with_name(path))
            .build()
            .context(format!("Failed to build config from {path}"))?
            .try_deserialize()
            .context(format!("Failed to deserialize config from {path}"))
    }

    pub fn current_path(&self) -> PathBuf {
        let today = Local::now();
        [
            &self.codual_path,
            "_log",
            &today.year().to_string(),
            &self.frequency.filename(&today),
        ]
        .iter()
        .collect()
    }
}
