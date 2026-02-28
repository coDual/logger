use anyhow::{Context, Result};
use serde::Deserialize;

/// Karakeep API configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub list_id: String,
}

/// A link bookmark with a resolved title.
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

// Raw API response types (private).
#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentSchema {
    #[serde(rename = "link")]
    Link { url: String, title: Option<String> },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct BookmarkSchema {
    title: Option<String>,
    content: ContentSchema,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseSchema {
    bookmarks: Vec<BookmarkSchema>,
    next_cursor: Option<serde_json::Value>,
}

/// Fetch all link bookmarks from a Karakeep list, handling cursor-based pagination.
pub async fn get_bookmarks(config: &Config) -> Result<Vec<Bookmark>> {
    let client = reqwest::Client::new();
    let mut bookmarks = Vec::new();
    let mut cursor: Option<String> = None;
    let url = format!(
        "{}/api/v1/lists/{}/bookmarks",
        config.base_url.trim_end_matches('/'),
        config.list_id
    );

    loop {
        let mut request = client.get(&url).bearer_auth(&config.api_key);
        if let Some(ref c) = cursor {
            request = request.query(&[("cursor", c.as_str())]);
        }

        let response: ResponseSchema = request
            .send()
            .await
            .context("Failed to send request to Karakeep API")?
            .error_for_status()
            .context("Karakeep API returned an error")?
            .json()
            .await
            .context("Failed to parse Karakeep API response")?;

        for raw in response.bookmarks {
            if let ContentSchema::Link {
                url,
                title: content_title,
            } = raw.content
            {
                bookmarks.push(Bookmark {
                    title: raw.title.or(content_title).unwrap_or_default(),
                    url,
                });
            }
        }

        match response.next_cursor {
            Some(serde_json::Value::String(c)) => cursor = Some(c),
            _ => break,
        }
    }

    Ok(bookmarks)
}
