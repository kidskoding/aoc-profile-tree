use reqwest::StatusCode;
use vercel_runtime::Response;
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

impl AocError {
    pub fn to_vercel_response(&self) -> Response<String> {
        let status = match self {
            AocError::InvalidSession | AocError::MissingSessionCookie => StatusCode::UNAUTHORIZED,
            AocError::Network(_) => StatusCode::BAD_GATEWAY,
            AocError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Response::builder()
            .status(status.as_u16())
            .header("Content-Type", "text/plain")
            .body(self.to_string())
            .expect("failed to build error response")
    }
}