use anyhow::Result;
use axum::{Router, routing::get};
use image::{ImageBuffer, ImageReader, Rgba};
use mcb::*;
use rand::seq::IndexedRandom;
use rand_chacha::ChaCha8Rng;
use std::{
    fs::{read_dir, read_to_string, write},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::handlers::*;

mod bitbanner;
mod generation;
mod handlers;
mod query;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub struct AppState {
    patterns: Vec<(String, Image)>,
    base: Image,
    banner_count: Mutex<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState {
        patterns: load_patterns("patterns")?,
        base: Banner::load_base()?,
        banner_count: Mutex::new(load_banner_count()?),
    };

    let app = Router::new()
        .route(
            "/",
            get(async || "Every place you've ever imagined, it's real"),
        )
        .route("/create", get(create_banner))
        .route("/banner", get(get_banner))
        .route("/banner/{seed}", get(get_banner))
        .route("/pattern", get(get_pattern_list))
        .route("/pattern/{seed}", get(get_pattern_list))
        .route("/seed", get(get_new_seed))
        .route("/metadata", get(get_metadata))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8213").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn map_base_color(base_color: Option<u8>) -> Option<Color> {
    base_color.map(|c| Color::from_repr(c)).flatten()
}

fn random_color(rng: &mut ChaCha8Rng) -> &Color {
    Color::all().choose(rng).unwrap()
}

fn load_patterns(dir: impl AsRef<std::path::Path>) -> Result<Vec<(String, Image)>> {
    let pattern_files = read_dir(&dir.as_ref())?;
    let mut patterns = Vec::new();

    for file in pattern_files {
        let file = file?;
        let id = file
            .file_name()
            .to_string_lossy()
            .split_once('.')
            .unwrap()
            .0
            .to_string();

        // this excludes the .gitkeep file
        if id.is_empty() {
            continue;
        }

        let img = ImageReader::open(file.path())?.decode()?.to_rgba8();

        patterns.push((id, img));
    }

    // make sure theyre consistently in the same order
    patterns.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(patterns)
}

const BANNER_STAT_FILE: &'static str = "count.txt";
fn load_banner_count() -> Result<u64> {
    let data = read_to_string(BANNER_STAT_FILE).unwrap_or("0".to_string());
    Ok(data.parse::<u64>()?)
}
fn save_banner_count(num: u64) -> Result<()> {
    write(BANNER_STAT_FILE, num.to_string())?;
    Ok(())
}
async fn increment_banner_count(num: &Mutex<u64>) {
    let mut num = num.lock().await;
    *num += 1;
    if *num % 100 == 0 {
        // ignore error
        match save_banner_count(*num) {
            Err(e) => {
                println!("{e:?}");
            }
            _ => (),
        }
    }
}

fn banner_from_pattern_list(
    rng: &mut ChaCha8Rng,
    base: &mut Image,
    base_color: Option<Color>,
    patterns: Vec<(usize, Color)>,
    pattern_ref: &Vec<(String, Image)>,
) -> Result<Image> {
    let base_color = match base_color {
        Some(color) => color,
        None => *random_color(rng),
    };
    let mut banner = Banner::new(base, base_color)?;

    for (pattern_id, color) in patterns {
        let pattern = Pattern::new(pattern_ref[pattern_id].1.clone());
        banner.add_pattern(pattern, &color)?;
    }

    Ok(banner.img_owned())
}
