use std::fs;

use aoc_profile_tree::{generate_svg, get_calendar_html};
use chrono::{Datelike, Local};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, env = "AOC_YEAR")]
    year: Option<String>,

    #[arg(short, long, env = "AOC_SESSION")]
    session: Option<String>,

    #[arg(short, long, default_value = "aoc_tree.svg")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let css = include_str!("../assets/style.css");

    let session = args.session.expect("AOC_SESSION must be provided");
    let year = args.year.unwrap_or_else(|| get_current_year().to_string());

    match get_calendar_html(&year, &session) {
        Ok(html) => {
            let svg = generate_svg(&html, css);
            fs::write(&args.output, svg)
                .expect("unable to write file");
            println!("Successfully generated tree for {}!", year);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_current_year() -> i32 {
    let now = Local::now();
    now.year()
}