use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("could not find the calendar art! is your AOC_SESSION valid?")]
    InvalidSession,

    #[error("failed to write the SVG file: {0}")]
    Io(#[from] std::io::Error),

    #[error("missing AOC_SESSION cookie!")]
    MissingSessionCookie,
}