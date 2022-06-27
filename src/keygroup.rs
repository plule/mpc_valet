use std::{
    hash::{Hash, Hasher},
    ops::RangeInclusive,
};

use egui::Color32;
use itertools::Itertools;
use music_note::midi::MidiNote;
use random_color::{Luminosity, RandomColor};

use crate::{Layer, LayerVelocityMode};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Keygroup {
    pub range: Option<RangeInclusive<MidiNote>>,
    pub layers: [Option<Layer>; 4],
}

impl PartialOrd for Keygroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.layers
            .get(0)
            .as_ref()
            .partial_cmp(&other.layers.get(0).as_ref())
    }
}

impl Ord for Keygroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.layers
            .get(0)
            .as_ref()
            .cmp(&other.layers.get(0).as_ref())
    }
}

impl Keygroup {
    pub fn new(range: RangeInclusive<MidiNote>, layers: [Option<Layer>; 4]) -> Self {
        Self {
            range: Some(range),
            layers,
        }
    }

    pub fn from_file(file: String) -> Self {
        Self {
            range: None,
            layers: [Some(Layer::from_file(file)), None, None, None],
        }
    }

    pub fn first_assigned_layer(&self) -> Option<&Layer> {
        self.layers.iter().find_map(|layer| layer.as_ref())
    }

    pub fn first_assigned_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.iter_mut().find_map(|layer| layer.as_mut())
    }

    pub fn color(&self) -> Color32 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let layer = self.first_assigned_layer().cloned().unwrap_or_default();

        layer.file.hash(&mut hasher);
        let color = RandomColor::new()
            .seed(hasher.finish())
            .luminosity(Luminosity::Light)
            .to_rgb_array();
        Color32::from_rgb(color[0], color[1], color[2])
    }

    pub fn layer_count(&self) -> usize {
        self.layers.iter().filter_map(|l| l.as_ref()).count()
    }

    pub fn set_velocity_layer_mode(&mut self, mode: &LayerVelocityMode) {
        match mode {
            LayerVelocityMode::Overlapping => {
                for layer in self.layers.iter_mut().filter_map(|l| l.as_mut()) {
                    layer.velocity = 0..=127;
                }
            }
            LayerVelocityMode::Spread => {
                let active_layers = self
                    .layers
                    .iter_mut()
                    .filter_map(|l| l.as_mut())
                    .collect_vec();
                let layer_count = active_layers.len();
                if layer_count == 0 {
                    return;
                }

                for (index, mut layer) in active_layers.into_iter().enumerate() {
                    let start: u8 = (128 * index / layer_count)
                        .try_into()
                        .expect("The lower bound velocity went out of bound");
                    let end: u8 = ((128 * (index + 1) / layer_count) - 1)
                        .try_into()
                        .expect("The upper bound velocity went out of bound");
                    layer.velocity = start..=end;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use music_note::midi;
    use rstest::rstest;

    pub use super::*;

    #[rstest]
    #[case(
        [None, None, None, None],
        [None, None, None, None],
        [None, None, None, None],
    )]
    #[case(
        [Some(Layer::default()), None, None, None],
        [Some(0..=127), None, None, None],
        [Some(0..=127), None, None, None],
    )]
    #[case(
        [Some(Layer::default()), Some(Layer::default()), None, None],
        [Some(0..=63), Some(64..=127), None, None],
        [Some(0..=127), Some(0..=127), None, None],
    )]
    #[case(
        [Some(Layer::default()), None, Some(Layer::default()), None],
        [Some(0..=63), None, Some(64..=127), None],
        [Some(0..=127), None, Some(0..=127), None],
    )]
    #[case(
        [Some(Layer::default()), Some(Layer::default()), Some(Layer::default()), None],
        [Some(0..=41), Some(42..=84), Some(85..=127), None],
        [Some(0..=127), Some(0..=127), Some(0..=127), None],
    )]
    #[case(
        [Some(Layer::default()), Some(Layer::default()), Some(Layer::default()), Some(Layer::default())],
        [Some(0..=31), Some(32..=63), Some(64..=95), Some(96..=127)],
        [Some(0..=127), Some(0..=127), Some(0..=127), Some(0..=127)],
    )]
    fn layer_velocity_test(
        #[case] layers: [Option<Layer>; 4],
        #[case] spread_velocity: [Option<RangeInclusive<u8>>; 4],
        #[case] overlap_velocity: [Option<RangeInclusive<u8>>; 4],
    ) {
        let mut kg = Keygroup::new(midi!(A, 2)..=midi!(A, 3), layers);

        kg.set_velocity_layer_mode(&LayerVelocityMode::Spread);

        let actual_spread_velocity = kg
            .layers
            .clone()
            .map(|layer| -> Option<RangeInclusive<u8>> { Some(layer?.velocity) });

        assert_eq!(spread_velocity, actual_spread_velocity);

        kg.set_velocity_layer_mode(&LayerVelocityMode::Overlapping);

        let actual_overlap_velocity = kg
            .layers
            .clone()
            .map(|layer| -> Option<RangeInclusive<u8>> { Some(layer?.velocity) });

        assert_eq!(overlap_velocity, actual_overlap_velocity);
    }
}
