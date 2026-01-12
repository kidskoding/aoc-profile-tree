pub mod error;

use reqwest::header::{COOKIE, HeaderMap};
use scraper::{Html, Selector};

use crate::error::AocError;

pub fn get_calendar_html(year: &str, session: &str) -> Result<String, AocError> {
    let url = format!("https://adventofcode.com/{}", year);

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE, 
        format!("session={}", session).parse().unwrap()
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .user_agent("rust-aoc-profile-scraper-by-user")
        .build()?;

    let response = client.get(url).send()?;
    if !response.status().is_success() {
        return Err(AocError::InvalidSession);
    }

    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("pre.calendar").map_err(|_| AocError::InvalidSession)?;

    let calendar = document.select(&selector)
        .next()
        .ok_or(AocError::InvalidSession)?;

    Ok(calendar.inner_html())
}

pub fn generate_svg(calendar_html: &str, css: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="450">
            <foreignObject width="100%" height="100%">
                <div xmlns="http://www.w3.org/1999/xhtml">
                    <style>{}</style>
                    <pre class="calendar">{}</pre>
                </div>
            </foreignObject>
        </svg>"#,
        css, calendar_html
    )
}