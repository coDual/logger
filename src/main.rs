use clap::Clap;
use std::convert::TryFrom;
use std::error;
use std::time::{SystemTime, UNIX_EPOCH};
use wallabag_api::types::{EntriesFilter, SortBy, SortOrder};
use wallabag_api::Client;

mod settings;
use settings::Settings;

#[derive(Clap)]
struct Opts {
    config: String,
}

async fn run_example() -> Result<(), Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();
    let settings = Settings::from_file(&opts.config)?;
    let mut client = Client::new(settings.wallabag.clone().into());

    // Only get starred entries
    let filter = EntriesFilter {
        archive: None,
        starred: Some(true),
        sort: SortBy::Created,
        order: SortOrder::Desc,
        tags: vec![],
        since: settings.get_ts()?,
        public: None,
        per_page: None,
    };

    let response = client.get_entries_with_filter(&filter).await;
    match response {
        Err(e) => {
            println!("Error: {}", e);
        }
        Ok(entries) => {
            // do something with the entries!
            for entry in entries {
                println!(
                    "{} | {} | Starred at {}",
                    entry.id,
                    entry.title.unwrap_or_else(|| "Untitled".to_owned()),
                    entry.starred_at.unwrap()
                );
            }
        }
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    settings.set_ts(i64::try_from(now)?)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    async_std::task::block_on(run_example())
}
