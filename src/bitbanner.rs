#![allow(dead_code)]

use mcb::{Banner, Color, Pattern};
use rand::{Rng, SeedableRng};

use crate::AppState;

// first 4 bits is the base color (4 bits = 15 max value, 0..=15, accounts for all colors)
// [0..=3]
// one pattern is [[0..=6], [7..=10]]
// where the first 6 bits is the pattern id, and the next 4 is the color, cobmined 10 bits
// so 6 layers = 6 * 10 bits = 60 bits + 4 bits for the base = 64 bits (u64)
// a pattern where the first 6 bits is all 0s is classified as "no pattern"
pub struct BannerId(u64);
impl BannerId {
    pub fn new() -> BannerId {
        Self(0)
    }

    pub fn set_base_color(&mut self, num: u8) {
        assert!(num < 16);
        let value = (num as u64 & 0xF) << 60;
        self.0 = (self.0 & !(0xFu64 << 60)) | value;
    }

    pub fn get_base_color(&self) -> u8 {
        ((self.0 >> 60) & 0xF) as u8
    }

    pub fn set_layer(&mut self, layer: u8, pattern: u8, color: u8) {
        assert!(color < 16 && layer < Banner::VANILLA_MAX_PATTERN_SIZE as u8 && pattern < 64);
        let packed = ((pattern as u64) << 4) | (color as u64);
        let shift = 60 - 10 * (layer + 1);
        let mask = !(0x3FFu64 << shift);
        self.0 = (self.0 & mask) | (packed << shift);
    }

    pub fn get_layer(&self, layer: u8) -> (u8, u8) {
        assert!(layer < Banner::VANILLA_MAX_PATTERN_SIZE as u8);

        let shift = 60 - 10 * (layer + 1);
        let layer = (self.0 >> shift) & 0x3FF;
        let pattern = ((layer >> 4) & 0x3F) as u8;
        let color = (layer & 0xF) as u8;

        (pattern, color)
    }
}

impl std::fmt::Debug for BannerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",
            self.get_base_color(),
            self.get_layer(0),
            self.get_layer(1),
            self.get_layer(2),
            self.get_layer(3),
            self.get_layer(4),
            self.get_layer(5)
        )
    }
}

// i honeslty have no clue if these are 100% correct (compute_layer_counts & compute_cumulative_counts mostly)
fn compute_layer_counts(pattern_len: usize) -> [u64; Banner::VANILLA_MAX_PATTERN_SIZE + 1] {
    let mut counts = [0u64; Banner::VANILLA_MAX_PATTERN_SIZE + 1];
    let mut power: u64 = 1;
    let base = (pattern_len * Color::all().len()) as u64;
    for i in 0..=6 {
        counts[i] = power;
        if i < 6 {
            power *= base;
        }
    }

    counts
}
fn compute_cumulative_counts(
    counts: &[u64; Banner::VANILLA_MAX_PATTERN_SIZE + 1],
) -> [u64; Banner::VANILLA_MAX_PATTERN_SIZE + 1] {
    let mut cumulative = [0u64; Banner::VANILLA_MAX_PATTERN_SIZE + 1];
    let mut sum = 0u64;
    for i in 0..=6 {
        cumulative[i] = sum;
        sum += counts[i];
    }
    cumulative
}

fn generate_random_banner<R: rand::RngCore>(rng: &mut R, pattern_len: usize) -> BannerId {
    let mut banner = BannerId::new();
    banner.set_base_color(rng.random_range(0..Color::all().len()) as u8);

    let counts = compute_layer_counts(pattern_len);
    let cumulative = compute_cumulative_counts(&counts);
    let total_banners =
        cumulative[Banner::VANILLA_MAX_PATTERN_SIZE] + counts[Banner::VANILLA_MAX_PATTERN_SIZE];

    let mut index = rng.random_range(0..total_banners);

    let mut num_layers = 0;
    for i in 0..=Banner::VANILLA_MAX_PATTERN_SIZE {
        if index < cumulative[i] + counts[i] {
            num_layers = i;
            index -= cumulative[i];
            break;
        }
    }

    for i in 0..num_layers {
        let digit = index % (pattern_len * Color::all().len()) as u64;
        index /= (pattern_len * Color::all().len()) as u64;

        let pattern_id = (digit / Color::all().len() as u64) as u8;
        let color = (digit % Color::all().len() as u64) as u8;

        banner.set_layer(i as u8, pattern_id, color);
    }

    banner
}

pub fn test(state: AppState) -> anyhow::Result<()> {
    let mut rng = rand_xorshift::XorShiftRng::from_os_rng();

    let time = std::time::Instant::now();
    let banner = generate_random_banner(&mut rng, state.patterns.len());
    println!(
        "{:?}: {banner:?} ({}, {:b})",
        time.elapsed(),
        banner.0,
        banner.0
    );

    let mut banner_img = Banner::new(
        &mut state.base.clone(),
        Color::from_repr(banner.get_base_color()).unwrap(),
    )?;

    for i in 0..6 {
        let (pattern, color) = banner.get_layer(i);
        let pattern = Pattern::new(state.patterns[pattern as usize].1.clone());
        banner_img.add_pattern(pattern, &Color::from_repr(color).unwrap())?;
    }
    banner_img.save("banner.png")?;

    Ok(())
}
