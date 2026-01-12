pub mod error;

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
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="500">
            <foreignObject width="100%" height="100%">
                <div xmlns="http://www.w3.org/1999/xhtml">
                    <style>{}</style>
                    <div class="calendar-wrapper">
                        <h2 class="year-title">Advent of Code {}</h2>
                        <pre class="calendar">{}</pre>
                    </div>
                </div>
            </foreignObject>
        </svg>"#,
        css, year, calendar_html
    )
}