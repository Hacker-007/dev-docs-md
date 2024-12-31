use std::{fs, io::BufReader};

use reqwest::blocking::Client;
use serde::{Deserialize, Deserializer};

use crate::{
    deserialize::StreamingDocsDBVisitor,
    error::{DocsError, DocsResult},
};

const AVAILABLE_DOCS_SET_URL: &str = "https://devdocs.io/docs.json";
const DOCS_DB_JSON: &str = "https://documents.devdocs.io";
const MODIFIED_TIME_FILE: &str = ".mtime.txt";

pub struct DocsClient {
    http_client: Client,
}

impl Default for DocsClient {
    fn default() -> Self {
        Self {
            http_client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DocsSetEntry {
    slug: String,
    #[serde(rename = "mtime")]
    modified_time: u64,
}

impl DocsSetEntry {
    pub fn slug(&self) -> &str {
        &self.slug
    }
}

impl DocsClient {
    pub fn fetch_available_entries(&self) -> DocsResult<Vec<DocsSetEntry>> {
        self.http_client
            .get(AVAILABLE_DOCS_SET_URL)
            .send()?
            .json()
            .map_err(Into::into)
    }

    pub fn fetch_entry(&self, entry: &DocsSetEntry) -> DocsResult<()> {
        let mut path = dirs::home_dir().ok_or_else(|| DocsError::Os)?;
        path.push(".docs");
        path.push(&entry.slug);
        let mtime_path = path.join(MODIFIED_TIME_FILE);
        let (already_fetched, mtime) = fs::read_to_string(&mtime_path)
            .ok()
            .and_then(|mtime| mtime.parse::<u64>().ok())
            .map(|mtime| (true, mtime))
            .unwrap_or((false, 0));

        let message = if already_fetched {
            "updating"
        } else {
            "downloading"
        };

        if mtime < entry.modified_time {
            let slug_db = self
                .http_client
                .get(format!(
                    "{}/{}/db.json?{}",
                    DOCS_DB_JSON, entry.slug, entry.modified_time
                ))
                .send()?;

            let slug_db = BufReader::new(slug_db);
            println!("{} {} docs", message, entry.slug);
            let mut deserializer = serde_json::Deserializer::from_reader(slug_db);
            let visitor = StreamingDocsDBVisitor { base_path: path };
            deserializer
                .deserialize_map(visitor)
                .map_err(|_| DocsError::Deserialization)?;

            fs::write(&mtime_path, format!("{}", entry.modified_time))?;
            println!("finished {} {} docs", message, entry.slug);
        }
        Ok(())
    }
}

// AVAILABLE_DOCS_SET_URL => all slugs with available docs
// DB_JSON_URL => get html content for every "chapter" of a slug's docs
// INDEX_JSON_URL => provides TOB for a slug's doc

// For a given slug, get the HTML contents and file name
//     stream contents read from the slug's db.json to corresponding HTML file
//
// When searching, look through index.json to find the path we should search
// then render this file as a Markdown file to STDOUT.
