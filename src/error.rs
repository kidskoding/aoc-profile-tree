use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("could not find the calendar art! Is your AOC_SESSION valid?")]
    InvalidSession,

    #[error("failed to write the SVG file: {0}")]
    Io(#[from] std::io::Error),

    #[error("missing AOC_SESSION cookie!")]
    MissingSessionCookie,
}

impl IntoResponse for AocError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AocError::InvalidSession | AocError::MissingSessionCookie => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            AocError::Network(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AocError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}