use anyhow::{Result, anyhow};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use axum_extra::extract::Query;
use image::{ImageFormat, ImageReader};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;
use serde_json::json;
use std::{
    fs::read_dir,
    io::{BufWriter, Cursor},
    sync::Arc,
};
use strum::VariantArray;

use crate::{generation::*, mcb::*};
mod generation;
mod mcb;

#[derive(Debug)]
pub struct AppState {
    patterns: Vec<(String, Image)>,
    base: Image,
}

#[tokio::main]
async fn main() -> Result<()> {
    let patterns = load_patterns("patterns")?;
    let base = Banner::load_base()?;

    let state = AppState { patterns, base };

    let app = Router::new()
        .route("/", get(root))
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

async fn root() -> &'static str {
    "Every place you've ever imagined, it's real"
}

#[derive(Debug, Deserialize)]
struct LayerEntry {
    id: Option<usize>,
    color: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct GetBannerQuery {
    base_color: Option<u8>,
    #[serde(default)]
    layers: Vec<Option<String>>,
    max_layers: Option<usize>,
}

fn get_rng_from_seed(seed: Option<Path<String>>, pattern_len: usize) -> Result<ChaCha8Rng> {
    let seed = seed
        .map(|s| match s.parse::<u64>() {
            Ok(s) => Some(s),
            Err(_) => None,
        })
        .flatten()
        .unwrap_or_else(|| generate_seed(pattern_len));

    let possible_combs = get_possible_combinations(pattern_len);
    if seed > possible_combs {
        return Err(anyhow!(
            "Seed is too big, must be less than {possible_combs}"
        ));
    }

    Ok(ChaCha8Rng::seed_from_u64(seed))
}

// [,]
// []
// [,5]
// ["", 5]
// ["",]
fn parse_layer_entry(s: &str) -> LayerEntry {
    let input = s.trim();
    // remove []
    let input = input.trim_end_matches(']');
    let input = input.trim_start_matches('[');
    // split at ,
    let (id, color) = match input.split_once(',') {
        Some(d) => d,
        None => {
            return LayerEntry {
                id: None,
                color: None,
            };
        }
    };
    let (id, color) = (id.trim(), color.trim());

    let id = if id.is_empty() {
        None
    } else {
        match id.parse::<usize>() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    };

    let color = if color.is_empty() {
        None
    } else {
        match color.parse::<u8>() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    };

    LayerEntry { id, color }
}

fn map_layers(layers: Vec<Option<String>>) -> Vec<Option<(Option<usize>, Option<Color>)>> {
    layers
        .into_iter()
        .map(|l| {
            let unit = match l {
                Some(l) => {
                    let entry = parse_layer_entry(&l);

                    let color = match entry.color {
                        Some(c) => match Color::from_repr(c) {
                            Some(c) => Some(c),
                            None => None,
                        },
                        None => None,
                    };

                    Some((entry.id, color))
                }
                None => None,
            };

            unit
        })
        .collect::<Vec<Option<(Option<usize>, Option<Color>)>>>()
}

fn map_base_color(base_color: Option<u8>) -> Option<Color> {
    base_color.map(|c| Color::from_repr(c)).flatten()
}

async fn get_banner(
    seed: Option<Path<String>>,
    Query(query): Query<GetBannerQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut rng = match get_rng_from_seed(seed, state.patterns.len()) {
        Ok(rng) => rng,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}"))),
    };

    let base_color = map_base_color(query.base_color);
    let layers = map_layers(query.layers);

    let pattern_list =
        match generate_pattern_list(&mut rng, &state.patterns, layers, query.max_layers) {
            Ok(i) => i,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
            }
        };
    let img = Banner::from_pattern_list(
        &mut rng,
        state.base.clone(),
        base_color,
        pattern_list,
        &state.patterns,
    )
    .unwrap();

    let mut buf = BufWriter::new(Cursor::new(vec![]));
    img.write_to(&mut buf, ImageFormat::WebP).unwrap();

    let bytes = buf.into_inner().unwrap().into_inner();
    let headers = [(header::CONTENT_TYPE, "image/webp")];

    Ok((headers, bytes))
}

async fn get_pattern_list(
    seed: Option<Path<String>>,
    Query(query): Query<GetBannerQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut rng = match get_rng_from_seed(seed, state.patterns.len()) {
        Ok(rng) => rng,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}"))),
    };

    let base_color = map_base_color(query.base_color);

    let layers = map_layers(query.layers);

    let pattern_list =
        match generate_pattern_list(&mut rng, &state.patterns, layers, query.max_layers) {
            Ok(i) => i,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
            }
        };
    let pattern_list = pattern_list
        .into_iter()
        .map(|(pattern_id, color)| {
            let pattern = state.patterns[pattern_id].0.to_owned();
            (pattern, color.to_string())
        })
        .collect::<Vec<(String, String)>>();

    // important we do base_color AFTER patterns, the same as in get_banner
    // so the seed is the same since its seq
    let base_color = match base_color {
        Some(color) => color,
        None => *Color::random(&mut rng),
    };

    Ok(Json(json!({
        "base": base_color.to_string(),
        "patterns": pattern_list
    })))
}

async fn get_new_seed(State(state): State<Arc<AppState>>) -> String {
    generate_seed(state.patterns.len()).to_string()
}

async fn get_metadata(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let patterns = state
        .patterns
        .iter()
        .map(|p| p.0.to_owned())
        .collect::<Vec<String>>();
    let colors = Color::VARIANTS
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    Json(json!({
        "patterns": patterns,
        "colors": colors,
        "combinations": get_possible_combinations(state.patterns.len()).to_string()
    }))
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

        let img = ImageReader::open(file.path())?.decode()?.to_rgba8();

        patterns.push((id, img));
    }

    Ok(patterns)
}
