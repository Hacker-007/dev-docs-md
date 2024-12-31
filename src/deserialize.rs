use std::{fs, path::PathBuf};

use html2md::parse_html;
use serde::de::{self, Visitor};
use tqdm::pbar;

pub struct StreamingDocsDBVisitor {
    pub base_path: PathBuf,
}

impl<'de> Visitor<'de> for StreamingDocsDBVisitor {
    type Value = ();

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a JSON map of file name to HTML file")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut progress_bar = pbar(None);
        while let Some((doc_path, html_docs)) = access.next_entry::<String, String>()? {
            let mut path = self.base_path.join(doc_path);
            let markdown_docs = parse_html(&html_docs);
            path.set_extension("md");
            fs::create_dir_all(path.parent().unwrap()).map_err(de::Error::custom)?;
            fs::write(path, markdown_docs).map_err(de::Error::custom)?;
            progress_bar.update(1).map_err(de::Error::custom)?;
        }

        Ok(())
    }
}
