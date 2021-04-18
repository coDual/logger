use clap::Clap;
use config::{Config, File};
use wallabag_api::types::{self, EntriesFilter, SortBy, SortOrder};
use wallabag_api::Client;

mod secrets;

#[derive(Clap)]
struct Opts {
    config: String,
}

async fn run_example() {
    let opts: Opts = Opts::parse();
    let mut settings = Config::default();
    settings
        .merge(File::with_name(opts.config.as_str()))
        .unwrap();
    let secrets = settings.try_into::<secrets::Secrets>().unwrap();

    let config = types::Config {
        client_id: secrets.client_id,
        client_secret: secrets.client_secret,
        username: secrets.username,
        password: secrets.password,
        base_url: secrets.base_url,
    };

    println!("{:#?}", config);

    let mut client = Client::new(config);

    // Only get starred entries
    let filter = EntriesFilter {
        archive: None,
        starred: Some(true),
        sort: SortBy::Created,
        order: SortOrder::Desc,
        tags: vec![],
        since: 0,
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
}

fn main() {
    async_std::task::block_on(run_example())
}
