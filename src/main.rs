use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::env;
use std::collections::HashMap;

use aoc_profile_tree::{error::AocError, generate_svg, get_calendar_html};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .route("/render", get(render));

    Ok(router.into())
}

async fn index() -> &'static str {
    "binary built!"
}

async fn render(Query(params): Query<HashMap<String, String>>) -> Response {
    let year = params.get("year").map(|s| s.as_str()).unwrap_or("2024");
    
    let session = match env::var("AOC_SESSION") {
        Ok(s) => s,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                "missing AOC_SESSION cookie!"
            ).into_response();
        }
    };

    let css = include_str!("../assets/style.css");

    match get_calendar_html(year, &session).await {
        Ok(html) => {
            let svg = generate_svg(&html, css, year);
            (
                StatusCode::OK,
                [("Content-Type", "image/svg+xml")],
                svg
            ).into_response()
        },
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error: {}", e)
            ).into_response()
        }
    }
}