use anyhow::{Result, anyhow};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use strum::VariantArray;

use crate::mcb::{Banner, Color, Image};

pub fn get_possible_combinations(pattern_len: usize) -> u64 {
    const C: u64 = Color::VARIANTS.len() as u64;
    let p = pattern_len as u64;
    let r = (p * C) as u64;
    C * (r.pow(6) + r.pow(5) + r.pow(4) + r.pow(3) + r.pow(2) + r + 1)
}

pub fn generate_seed(pattern_len: usize) -> u64 {
    let mut rng = ChaCha8Rng::from_os_rng();
    rng.random_range(0..get_possible_combinations(pattern_len))
}

pub fn generate_pattern_list(
    rng: &mut ChaCha8Rng,
    pattern_ref: &Vec<(String, Image)>,
    layers: Vec<Option<(Option<usize>, Option<Color>)>>,
    max_layers: Option<usize>,
) -> Result<Vec<(usize, Color)>> {
    let mut patterns = Vec::with_capacity(6);
    for _ in 0..get_amount_of_layers(rng, pattern_ref.len()) {
        let pattern = rng.random_range(0..pattern_ref.len() - 1);
        let color = Color::random(rng);
        patterns.push((pattern, *color));
    }

    let set_patterns = layers;
    if set_patterns.len() > Banner::VANILLA_MAX_PATTERN_SIZE {
        return Err(anyhow!("Too many layers provided"));
    }

    // override any random pattern if a set one was provided
    for (i, set_p) in set_patterns.into_iter().enumerate() {
        if let Some((pattern_id, color)) = set_p {
            let new_pattern = match (pattern_id, color) {
                (None, None) => patterns[i],
                (Some(i), None) => (i, patterns[i].1),
                (None, Some(c)) => (patterns[i].0, c),
                (Some(i), Some(c)) => (i, c),
            };
            patterns[i] = new_pattern;
        }
    }

    // technically you can just build your own banners via this
    // it will still run the randomizer for all 6 layers etc
    // and you can then just override them all and or just a few
    // and then truncate it to the amount you want
    if let Some(max_layers) = max_layers {
        patterns.truncate(max_layers);
    }

    Ok(patterns)
}

fn get_amount_of_layers(rng: &mut ChaCha8Rng, pattern_len: usize) -> usize {
    const COLORS: usize = Color::VARIANTS.len();

    let mut total: u64 = 0;
    let mut counts = Vec::with_capacity(Banner::VANILLA_MAX_PATTERN_SIZE);

    for n in 0..=Banner::VANILLA_MAX_PATTERN_SIZE {
        let layer_combos = (pattern_len * COLORS) as u64;
        let count = COLORS as u64 * layer_combos.pow(n as u32);
        counts.push(count);
        total += count;
    }

    let choice = rng.random_range(0..total);

    let mut cumulative: u64 = 0;
    let mut num_layers = 0;
    for (i, &count) in counts.iter().enumerate() {
        if choice < cumulative + count {
            num_layers = i;
            break;
        }
        cumulative += count;
    }

    num_layers
}
