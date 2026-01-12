use std::{collections::HashMap, env};

use aoc_profile_tree::{error::AocError, generate_svg, get_calendar_html};
use reqwest::{StatusCode, Url};
use vercel_runtime::{Request, Response, run, service_fn};

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    run(service_fn(handler)).await
}

pub async fn handler(req: Request) -> Result<Response<String>, vercel_runtime::Error> {
    let base_url = Url::parse("http://localhost").unwrap();
    let parsed_url = base_url
        .join(&req.uri().to_string())
        .map_err(|e| vercel_runtime::Error::from(format!("invalid URI: {}", e)))?;

    let hash_query: HashMap<String, String> = parsed_url
        .query_pairs()
        .into_owned()
        .collect();

    let year = hash_query
        .get("year")
        .cloned()
        .unwrap_or_else(|| "2025".to_string());

    let session = match env::var("AOC_SESSION") {
        Ok(s) => s,
        Err(_) => return Ok(AocError::MissingSessionCookie.to_vercel_response()),
    };

    let css = include_str!("../assets/style.css");

    match get_calendar_html(&year, &session) {
        Ok(html) => {
            let svg = generate_svg(&html, css, &year);
            Ok(Response::builder()
                .status(StatusCode::OK.as_u16())
                .header("Content-Type", "image/svg+xml")
                .header("Cache-Control", "s-maxage=3600, stale-while-revalidate")
                .body(svg)?)
        },
        Err(e) => Ok(e.to_vercel_response()),
    }
}