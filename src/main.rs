use anyhow::{Context, Result};
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::{error, fmt::Display, path::Path};
use wallabag_api::types::{EntriesFilter, Entry, SortBy, SortOrder};
use wallabag_api::Client;

mod settings;
use settings::Settings;

#[derive(Parser)]
struct Opts {
    /// Configuration file
    #[clap(value_parser)]
    config: String,
}

// Get Wallabag starred posts.
async fn get_starred_posts(settings: &Settings) -> Result<Vec<Entry>> {
    let mut client = Client::new(settings.wallabag.clone().into());

    client
        .get_entries_with_filter(&EntriesFilter {
            archive: None,
            starred: Some(true),
            sort: SortBy::Created,
            order: SortOrder::Desc,
            tags: vec![],
            since: settings.ts()?,
            public: None,
            per_page: None,
        })
        .await
        .context("Failed to get Wallabag entries")
}

struct Link<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

impl<'a> Display for Link<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.title, self.url)
    }
}

// Print all starred Wallabag entries since last saved time.
async fn store_all_entries(log_path: &Path, entries: Vec<Entry>) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(log_path)
        .context(format!("Failed to open {0}", log_path.display()))?;

    for entry in entries {
        writeln!(
            file,
            "- {}",
            Link {
                title: &entry.title.unwrap_or_default(),
                url: &entry.url.unwrap_or_default(),
            }
        )?;
    }
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();
    let settings = Settings::from_file(&opts.config)?;
    let entries = get_starred_posts(&settings).await?;
    store_all_entries(&settings.current_path(), entries).await?;

    // Only update timestamp if entries were updated
    Ok(settings.update_ts()?)
}
