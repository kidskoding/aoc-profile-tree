extern crate shuttle_axum;

use std::{borrow::Cow, collections::HashMap};

use aoc_profile_tree::error::AocError;
use axum::{extract::Query, response::IntoResponse};
use chrono::{Datelike, Local};

async fn render_handler(Query(params): Query<HashMap<String, String>>) -> Result<impl IntoResponse, AocError> {
    let year: Cow<str> = params
        .get("year")
        .map(|s| Cow::Borrowed(s.as_str()))
        .unwrap_or_else(|| Cow::Owned(current_year().to_string()));

    let session = params
        .get("session")
        .ok_or(AocError::InvalidSession)?;

    let html = aoc_profile_tree::get_calendar_html(&year, &session)?;
    let css = include_str!("../assets/style.css");
    let svg = aoc_profile_tree::generate_svg(&html, css, &year);

    Ok((
        [
            (axum::http::header::CONTENT_TYPE, "image/svg+xml"),
            (axum::http::header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        svg,
    ))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/render", get(render_handler));
    Ok(router.into())
}

fn current_year() -> i32 {
    let localtime = Local::now();
    localtime.year()
}