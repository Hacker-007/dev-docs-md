use std::fmt::Display;

pub type DocsResult<T> = Result<T, DocsError>;

#[derive(Debug)]
pub enum DocsError {
    HttpError(reqwest::Error),
    OsError,
    DeserializationError,
}

impl From<reqwest::Error> for DocsError {
    fn from(err: reqwest::Error) -> Self {
        DocsError::HttpError(err)
    }
}

impl Display for DocsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocsError::HttpError(http_err) => write!(
                f,
                "an error occurred when sending an HTTP request to {}",
                http_err
                    .url()
                    .map(|url| url.as_str())
                    .unwrap_or("an unknown URL")
            ),
            DocsError::OsError => write!(f, "an error occurred when interacting with the OS"),
            DocsError::DeserializationError => write!(f, "an error occurred when deserializing input"),
        }
    }
}
