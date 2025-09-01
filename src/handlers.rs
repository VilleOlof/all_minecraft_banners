use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::Query;
use image::ImageFormat;
use mcb::*;
use serde_json::json;
use std::{
    io::{BufWriter, Cursor},
    sync::Arc,
};

use crate::{
    AppState, banner_from_pattern_list,
    generation::{
        generate_pattern_list, generate_seed, get_possible_combinations, get_rng_from_seed,
    },
    map_base_color,
    query::{GetBannerQuery, map_layers},
    random_color,
};

// add a ton of cache headers
pub async fn get_banner(
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
    let img = banner_from_pattern_list(
        &mut rng,
        &mut state.base.clone(),
        base_color,
        pattern_list,
        &state.patterns,
    )
    .unwrap();

    let mut buf = BufWriter::new(Cursor::new(vec![]));
    img.write_to(&mut buf, ImageFormat::WebP).unwrap();

    let bytes = buf.into_inner().unwrap().into_inner();
    let headers = [
        (header::CONTENT_TYPE, "image/webp"),
        (header::CACHE_CONTROL, "public, max-age=3600"),
    ];

    Ok((headers, bytes))
}

pub async fn get_pattern_list(
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
        None => *random_color(&mut rng),
    };

    Ok(Json(json!({
        "base": base_color.to_string(),
        "patterns": pattern_list
    })))
}

pub async fn get_new_seed(State(state): State<Arc<AppState>>) -> String {
    generate_seed(state.patterns.len()).to_string()
}

pub async fn get_metadata(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let patterns = state
        .patterns
        .iter()
        .map(|p| p.0.to_owned())
        .collect::<Vec<String>>();
    let colors = Color::all()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    Json(json!({
        "patterns": patterns,
        "colors": colors,
        "combinations": get_possible_combinations(state.patterns.len()).to_string()
    }))
}

pub async fn create_banner(
    Query(query): Query<GetBannerQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let base_color = match query.base_color {
        Some(color) => match Color::from_repr(color) {
            Some(c) => c,
            None => return Err((StatusCode::BAD_REQUEST, "Invalid 'base_color'".to_string())),
        },
        None => return Err((StatusCode::BAD_REQUEST, "Missing 'base_color'".to_string())),
    };
    let layers = map_layers(query.layers);

    let mut banner = match Banner::new(&mut state.base.clone(), base_color) {
        Ok(b) => b,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create banner".to_string(),
            ));
        }
    };

    for layer in layers {
        let layer = match layer {
            Some(l) => l,
            None => continue,
        };

        let (pattern_id, color) = match layer {
            (Some(i), Some(c)) => (i, c),
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid layer arguments".to_string(),
                ));
            }
        };
        let pattern = state.patterns[pattern_id].1.clone();
        let pattern = Pattern::new(pattern);
        match banner.add_pattern(pattern, &color) {
            Ok(_) => (),
            Err(_) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to add layer".to_string(),
                ));
            }
        };
    }

    let mut buf = BufWriter::new(Cursor::new(vec![]));
    banner.write_to(&mut buf, ImageFormat::WebP).unwrap();

    let bytes = buf.into_inner().unwrap().into_inner();
    let headers = [
        (header::CONTENT_TYPE, "image/webp"),
        (header::CACHE_CONTROL, "public, max-age=3600"),
    ];

    Ok((headers, bytes))
}
