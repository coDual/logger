use clap::Clap;
use std::error;
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

    Ok(client
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
        .await?)
}

// Print all starred Wallabag entries since last saved time.
async fn print_all_entries(entries: &mut Vec<Entry>) {
    // do something with the entries!
    for entry in entries {
        println!(
            "{} | {} | Starred at {}",
            entry.id,
            entry.title.get_or_insert_with(|| "Untitled".to_owned()),
            entry.starred_at.unwrap()
        );
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opts: Opts = Opts::parse();
    let settings = Settings::from_file(&opts.config)?;
    let mut entries = get_starred_posts(&settings).await?;
    print_all_entries(&mut entries).await;

    // Only update timestamp if entries were updated
    Ok(settings.update_ts()?)
}
