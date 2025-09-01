use mcb::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LayerEntry {
    pub id: Option<usize>,
    pub color: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct GetBannerQuery {
    pub base_color: Option<u8>,
    #[serde(default)]
    pub layers: Vec<Option<String>>,
    pub max_layers: Option<usize>,
}

// custom layer/pattern query format
// each pattern in a query consists of a pattern id and a color id
// the pattern id is which pattern index in the list
// and same as the color.
// some examples on how the input can be given:
// [,]
// []
// [,5]
// [9,]
// [1, 5]
pub fn parse_layer_entry(s: &str) -> LayerEntry {
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

pub fn map_layers(layers: Vec<Option<String>>) -> Vec<Option<(Option<usize>, Option<Color>)>> {
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
