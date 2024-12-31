use std::fmt::Display;

pub type DocsResult<T> = Result<T, DocsError>;

#[derive(Debug)]
pub enum DocsError {
    Http(reqwest::Error),
    Os,
    Deserialization,
    Unknown,
}

impl From<reqwest::Error> for DocsError {
    fn from(err: reqwest::Error) -> Self {
        DocsError::Http(err)
    }
}

impl From<std::io::Error> for DocsError {
    fn from(_: std::io::Error) -> Self {
        DocsError::Unknown
    }
}

impl Display for DocsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocsError::Http(http_err) => write!(
                f,
                "an error occurred when sending an HTTP request to {}",
                http_err
                    .url()
                    .map(|url| url.as_str())
                    .unwrap_or("an unknown URL")
            ),
            DocsError::Os => write!(f, "an error occurred when interacting with the OS"),
            DocsError::Deserialization => {
                write!(f, "an error occurred when deserializing input")
            }
            DocsError::Unknown => write!(f, "an unknown error occurred"),
        }
    }
}
