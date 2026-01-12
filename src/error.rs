use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Could not find the calendar art. Is your AOC_SESSION valid?")]
    InvalidSession,

    #[error("Failed to write the SVG file: {0}")]
    Io(#[from] std::io::Error),
}