use anyhow::{Context, Result};
use clap::Parser;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

mod karakeep;
mod settings;

#[derive(Parser)]
struct Opts {
    /// Configuration file
    #[clap(value_parser)]
    config: String,
}

struct Link<'a> {
    title: &'a str,
    url: &'a str,
}

impl Display for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]({})", self.title, self.url)
    }
}

fn store_all_entries(log_path: &Path, bookmarks: &[karakeep::Bookmark]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(log_path)
        .context(format!("Failed to open {}", log_path.display()))?;

    for bookmark in bookmarks {
        writeln!(
            file,
            "- {}",
            Link {
                title: &bookmark.title,
                url: &bookmark.url,
            }
        )?;
    }
    Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let settings = settings::Settings::from_file(&opts.config)?;
    let bookmarks = karakeep::get_bookmarks(&settings.karakeep).await?;
    store_all_entries(&settings.current_path(), &bookmarks)?;
    Ok(())
}
