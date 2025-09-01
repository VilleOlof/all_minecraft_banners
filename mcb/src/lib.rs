use image::{ImageBuffer, Rgba};
use std::ops::Deref;
use thiserror::Error;

mod color;
pub use color::Color;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Banner doesn't match the 20x40 image dimensions")]
    InvalidBannerDimension,
    #[error("The given hex color code isn't 6 in length")]
    MismatchedHexLength,
}

/// Nice short-hand for the image buffer type (rgba)
type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// A Minecraft Banner
pub struct Banner {
    image: Image,
}

impl Banner {
    /// Banner X width
    pub const X: u32 = 20;
    /// Banner Y height
    pub const Y: u32 = 40;

    /// How many patterns can be overlayed on a banner in Minecraft Survival mode.  
    pub const VANILLA_MAX_PATTERN_SIZE: usize = 6;
    /// The base texture used in all banners
    const BASE: &'static [u8] = include_bytes!("./base.png");

    /// Creates a new [`Banner`] from a base texture and a given [`Color`].  
    pub fn new(base: &mut Image, color: Color) -> Result<Self, Error> {
        let mut banner = Image::new(Banner::X, Banner::Y);

        tint_image(base, &color)?;
        image::imageops::overlay(&mut banner, base, 0, 0);

        Ok(Banner { image: banner })
    }

    /// Returns the base [`Image`] texture used for all banners
    ///
    /// Useful if creating many [`Banner`]s so you dont have to decode the texture every time.  
    pub fn load_base() -> Result<Image, Error> {
        let base = image::load_from_memory(Banner::BASE)?.to_rgba8();
        let base = Pattern::crop_pattern(&base);
        Ok(base)
    }

    /// Overlays a [`Pattern`] onto the [`Banner`].  
    ///
    /// This action is destructive and can't be reversed.  
    ///
    /// The given [`Color`] tints the [`Pattern`] before overlaying.  
    pub fn add_pattern(&mut self, pattern: Pattern, color: &Color) -> Result<(), Error> {
        if pattern.width() != Banner::X || pattern.height() != Banner::Y {
            return Err(Error::InvalidBannerDimension);
        }

        let mut pattern = pattern.img_owned();
        tint_image(&mut pattern, color)?;
        image::imageops::overlay(&mut self.image, &pattern, 0, 0);

        Ok(())
    }

    /// Returns the inner Banner [`Image`]
    pub fn img_owned(self) -> Image {
        self.image
    }
}

/// A Banner Pattern
pub struct Pattern {
    image: Image,
}

impl Pattern {
    /// Creates a new [`Pattern`] from a raw, full sized `64x64` pattern texture.  
    pub fn new(raw: Image) -> Self {
        Pattern {
            image: Pattern::crop_pattern(&raw),
        }
    }

    /// Crops a raw [`Pattern`] texture to extract only the front facing pattern texture.  
    fn crop_pattern(pattern: &Image) -> Image {
        image::imageops::crop_imm(pattern, 1, 1, Banner::X, Banner::Y).to_image()
    }

    /// Returns a reference to the inner [`Image`]
    pub fn img(&self) -> &Image {
        &self.image
    }

    /// Returns the inner [`Image`]
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

/// Tints an [`Image`] with the provided [`Color`].  
///
/// `(current.r as f32 * color.r as f32 / 255.0f32) as u8`
fn tint_image(data: &mut Image, color: &Color) -> Result<(), Error> {
    let (red, green, blue) = color::hex_to_rgb(&color.to_string())?;

    for pixel in data.pixels_mut() {
        pixel[0] = (pixel[0] as f32 * red as f32 / 255.0) as u8;
        pixel[1] = (pixel[1] as f32 * green as f32 / 255.0) as u8;
        pixel[2] = (pixel[2] as f32 * blue as f32 / 255.0) as u8;
    }

    Ok(())
}
