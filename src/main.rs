use std::env;
use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use shuttle_runtime::SecretStore;

use aoc_profile_tree::state::AppState;
use aoc_profile_tree::{generate_svg, get_calendar_html};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore
) -> shuttle_axum::ShuttleAxum {
    let aoc_session = secrets
        .get("AOC_SESSION")
        .expect("AOC_SESSION must be set in Secrets.toml");

    let state = AppState {
        aoc_session: aoc_session.to_string(),
    };

    let router = Router::new()
        .route("/", get(index))
        .route("/render", get(render))
        .with_state(state);

    Ok(router.into())
}

async fn index() -> &'static str {
    "binary built!"
}

async fn render(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let year = params
        .get("year")
        .map(|s| s.as_str())
        .unwrap_or("2025");
    let session = &state.aoc_session;
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
                format!("error: {}", e)
            ).into_response()
        }
    }
}