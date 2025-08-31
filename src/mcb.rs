use std::ops::Deref;

use anyhow::{Result, anyhow};
use image::{ImageBuffer, Rgba};
use rand::seq::IndexedRandom;
use rand_chacha::ChaCha8Rng;
use serde_repr::Serialize_repr;
use strum::{Display, FromRepr, VariantArray};

#[allow(dead_code)]
#[derive(
    Debug,
    Hash,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Display,
    FromRepr,
    VariantArray,
    Serialize_repr,
)]
#[repr(u8)]
pub enum Color {
    #[strum(to_string = "#F9FFFE")]
    White = 0,
    #[strum(to_string = "#F9801D")]
    Orange,
    #[strum(to_string = "#C74EBD")]
    Magenta,
    #[strum(to_string = "#3AB3DA")]
    LightBlue,
    #[strum(to_string = "#FED83D")]
    Yellow,
    #[strum(to_string = "#80C71F")]
    Lime,
    #[strum(to_string = "#F38BAA")]
    Pink,
    #[strum(to_string = "#474F52")]
    Gray,
    #[strum(to_string = "#9D9D97")]
    LightGray,
    #[strum(to_string = "#169C9C")]
    Cyan,
    #[strum(to_string = "#8932B8")]
    Purple,
    #[strum(to_string = "#3C44AA")]
    Blue,
    #[strum(to_string = "#835432")]
    Brown,
    #[strum(to_string = "#5E7C16")]
    Green,
    #[strum(to_string = "#B02E26")]
    Red,
    #[strum(to_string = "#1D1D21")]
    Black,
}

impl Color {
    pub fn random(rng: &mut ChaCha8Rng) -> &Self {
        Self::VARIANTS.choose(rng).unwrap()
    }
}

pub type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub const BANNER_BASE: &'static [u8] = include_bytes!("./base.png");

pub struct Banner {
    image: Image,
}
impl Banner {
    pub const X: u32 = 20;
    pub const Y: u32 = 40;

    pub const VANILLA_MAX_PATTERN_SIZE: usize = 6;

    pub fn new(base: Image, color: Color) -> Result<Self> {
        let mut banner = Image::new(Banner::X, Banner::Y);

        let tinted_base = tint_image(base, &color)?;
        image::imageops::overlay(&mut banner, &tinted_base, 0, 0);

        Ok(Banner { image: banner })
    }

    pub fn load_base() -> Result<Image> {
        let base = image::load_from_memory(BANNER_BASE)?.to_rgba8();
        let base = Pattern::crop_pattern(&base);
        Ok(base)
    }

    pub fn add_pattern(&mut self, pattern: Pattern, color: &Color) -> Result<()> {
        if pattern.width() != Banner::X || pattern.height() != Banner::Y {
            return Err(anyhow!(
                "Invalid banner dimensions ({}, {})",
                pattern.width(),
                pattern.height()
            ));
        }

        let tinted_pattern = tint_image(pattern.img_owned(), color)?;
        image::imageops::overlay(&mut self.image, &tinted_pattern, 0, 0);

        Ok(())
    }

    pub fn img_owned(self) -> Image {
        self.image
    }

    pub fn from_pattern_list(
        rng: &mut ChaCha8Rng,
        base: Image,
        base_color: Option<Color>,
        patterns: Vec<(usize, Color)>,
        pattern_ref: &Vec<(String, Image)>,
    ) -> Result<Image> {
        let base_color = match base_color {
            Some(color) => color,
            None => *Color::random(rng),
        };
        let mut banner = Banner::new(base, base_color)?;

        for (pattern_id, color) in patterns {
            let pattern = Pattern::new(pattern_ref[pattern_id].1.clone());
            banner.add_pattern(pattern, &color)?;
        }

        Ok(banner.img_owned())
    }
}

pub struct Pattern {
    image: Image,
}
impl Pattern {
    pub fn new(raw: Image) -> Self {
        Pattern {
            image: Pattern::crop_pattern(&raw),
        }
    }

    fn crop_pattern(pattern: &Image) -> Image {
        image::imageops::crop_imm(pattern, 1, 1, Banner::X, Banner::Y).to_image()
    }

    pub fn img(&self) -> &Image {
        &self.image
    }

    pub fn img_owned(self) -> Image {
        self.image
    }
}

impl Deref for Pattern {
    type Target = Image;
    fn deref(&self) -> &Self::Target {
        self.img()
    }
}

impl Deref for Banner {
    type Target = Image;
    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

fn tint_image(data: Image, color: &Color) -> Result<Image> {
    let mut data = data;
    let (red, green, blue) = hex_to_rgb(&color.to_string())?;

    for pixel in data.pixels_mut() {
        pixel[0] = (pixel[0] as f32 * red as f32 / 255.0) as u8;
        pixel[1] = (pixel[1] as f32 * green as f32 / 255.0) as u8;
        pixel[2] = (pixel[2] as f32 * blue as f32 / 255.0) as u8;
    }

    Ok(data)
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(anyhow!("Invalid hex length: {} != 6", hex.len()));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}
