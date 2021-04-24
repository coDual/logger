use clap::Clap;
use std::convert::TryFrom;
use std::error;
use std::time::{SystemTime, UNIX_EPOCH};
use wallabag_api::types::{EntriesFilter, Entry, SortBy, SortOrder};
use wallabag_api::Client;

mod settings;
use settings::Settings;

#[derive(Clap)]
struct Opts {
    config: String,
}

// Get Wallabag starred posts.
async fn get_starred_posts(settings: &Settings) -> Result<Vec<Entry>, anyhow::Error> {
    let mut client = Client::new(settings.wallabag.clone().into());

    let entries = client
        .get_entries_with_filter(&EntriesFilter {
            archive: None,
            starred: Some(true),
            sort: SortBy::Created,
            order: SortOrder::Desc,
            tags: vec![],
            since: settings.get_ts()?,
            public: None,
            per_page: None,
        })
        .await?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    settings.set_ts(i64::try_from(now)?)?;
    Ok(entries)
}

// Print all starred Wallabag entries since last saved time.
async fn print_all_entries(settings: &Settings) -> Result<(), Box<dyn error::Error>> {
    let entries = get_starred_posts(&settings).await?;

    // do something with the entries!
    for entry in entries {
        println!(
            "{} | {} | Starred at {}",
            entry.id,
            entry.title.unwrap_or_else(|| "Untitled".to_owned()),
            entry.starred_at.unwrap()
        );
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();
    let settings = Settings::from_file(&opts.config)?;
    async_std::task::block_on(print_all_entries(&settings))
}
