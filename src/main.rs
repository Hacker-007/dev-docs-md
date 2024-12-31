use clap::Parser;
use client::DocsClient;
use error::DocsError;

mod client;
mod deserialize;
mod error;

#[derive(Debug, clap::Parser)]
#[command(version)]
/// CLI to display Markdown-based documentation, sourced from
/// devdocs.io. Although there are other Rust crates that provide similar
/// functionality, they do not provide first-party support for Markdown
/// output.
///
/// # Examples
/// ```bash
/// docs search rust String::trim
/// ```
/// This command searches the Rust documentation for the `trim` function on
/// the `String` type. This also downloades the documentation to the local
/// machine if they could not be found.
enum DocsCommand {
    /// Searches for the given `query` within the given `language`'s
    /// documentation. If the associated documentation does not exist,
    /// it will be downloaded to the local device in the default configuration
    /// location (~/.docs)
    #[command(about = "Searches for the given `query` within the given `languages` docs")]
    Search { language: String, query: String },
}

fn process_command(command: DocsCommand) -> Result<(), DocsError> {
    match command {
        DocsCommand::Search { language, .. } => {
            let client = DocsClient::default();
            let entries = client.fetch_available_entries()?;
            let entry = entries
                .iter()
                .find(|entry| entry.slug() == language)
                .expect("no support for fuzzy matching yet");

            client.fetch_entry(entry)?;
        }
    }

    Ok(())
}

fn main() {
    let command: DocsCommand = DocsCommand::parse();
    if let Err(err) = process_command(command) {
        println!("[error] {}", err)
    }
}
