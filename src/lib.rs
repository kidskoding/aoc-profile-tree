pub mod error;
pub mod state;

use std::time::Duration;
use reqwest::header::{COOKIE, HeaderMap};
use scraper::{Html, Selector};

use crate::error::AocError;

pub async fn get_calendar_html(year: &str, session: &str) -> Result<String, AocError> {
    let url = format!("https://adventofcode.com/{}", year);

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE, 
        format!("session={}", session).parse().unwrap()
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent("rust-aoc-profile-scraper")
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| AocError::InvalidSession)?;

    let response = client.get(url)
        .send()
        .await
        .map_err(|_| AocError::InvalidSession)?;

    if !response.status().is_success() {
        return Err(AocError::InvalidSession);
    }

    let html_content = response.text()
        .await
        .map_err(|_| AocError::InvalidSession)?;

    let document = Html::parse_document(&html_content);
    let selector = Selector::parse("pre.calendar")
        .map_err(|_| AocError::InvalidSession)?;

    let calendar = document.select(&selector)
        .next()
        .ok_or(AocError::InvalidSession)?;

    Ok(calendar.inner_html())
}

pub fn generate_svg(calendar_html: &str, css: &str, year: &str) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="800">
            <rect width="100%" height="100%" fill="#0f0f23"/>
            <foreignObject width="100%" height="100%">
                <div xmlns="http://www.w3.org/1999/xhtml">
                    <style>
                        @import url("https://fonts.googleapis.com/css2?family=Source+Code+Pro&amp;display=swap");

                        .calendar, .calendar * {{
                            background: transparent !important;
                            border: none !important;
                            text-decoration: none !important;
                            box-shadow: none !important;
                            outline: none !important;
                            color: inherit; /* Prevent links from being "Link Blue" */
                        }}

                        pre {{
                            white-space: pre !important;
                            margin: 0;
                            padding: 20px;
                            font-family: "Source Code Pro", monospace;
                            color: #cccccc;
                        }}

                        .calendar-mark-complete, 
                        .calendar-mark-verycomplete, 
                        .calendar-color-s {{ 
                            color: #ffff66 !important; 
                            text-shadow: 0 0 5px #ffff66, 0 0 10px #ffff66 !important; 
                        }}

                        {}
                    </style>
                    <div class="calendar">
                        <pre>{}</pre>
                    </div>
                </div>
            </foreignObject>
        </svg>"##,
        css,
        calendar_html
    )
}