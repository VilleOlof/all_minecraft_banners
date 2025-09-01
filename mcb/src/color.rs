use crate::Error;
use serde_repr::Serialize_repr;
use strum::{Display, FromRepr, VariantArray};

/// Every Minecraft dye [Color](https://minecraft.wiki/w/Dye).
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
    /// Returns all [`Color`]s
    pub const fn all() -> &'static [Self] {
        Color::VARIANTS
    }
}

/// Simply converts a `&str` of **Hex** color into **RGB** (u8, u8, u8).  
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), Error> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(Error::MismatchedHexLength);
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}
