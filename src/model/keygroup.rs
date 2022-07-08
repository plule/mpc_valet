use std::ops::RangeInclusive;

use itertools::Itertools;
use music_note::midi::MidiNote;

use super::{Layer, LayerVelocityMode};

/// A keygroup is a set of samples assign to a note range on a keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keygroup {
    /// Note range where this keygroup is active.
    pub range: RangeInclusive<MidiNote>,

    /// Layers of the keygroup.
    ///
    /// Each layer can hold a sound sample and be of its own root note.
    pub layers: [Option<Layer>; 4],
}

impl Default for Keygroup {
    fn default() -> Self {
        Self {
            range: MidiNote::from_byte(0)..=MidiNote::from_byte(127),
            layers: Default::default(),
        }
    }
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
        Self { range, layers }
    }

    /// Get the first layer with an assigned sample.
    pub fn first_assigned_layer(&self) -> Option<&Layer> {
        self.layers.iter().find_map(|layer| layer.as_ref())
    }

    /// Get the first layer with an assigned sample (mutable).
    pub fn first_assigned_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.iter_mut().find_map(|layer| layer.as_mut())
    }

    /// Number of assigned layers.
    pub fn layer_count(&self) -> usize {
        self.layers.iter().filter_map(|l| l.as_ref()).count()
    }

    /// Choose the way the velocity range should be assigned accross the layers.
    pub fn set_velocity_layer_mode(&mut self, mode: &LayerVelocityMode) {
        match mode {
            LayerVelocityMode::Unison => {
                for layer in self.layers.iter_mut().filter_map(|l| l.as_mut()) {
                    layer.velocity = 0..=127;
                }
            }
            LayerVelocityMode::Automatic => {
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
        #[case] automatic_velocity: [Option<RangeInclusive<u8>>; 4],
        #[case] unison_velocity: [Option<RangeInclusive<u8>>; 4],
    ) {
        let mut kg = Keygroup::new(midi!(A, 2)..=midi!(A, 3), layers);

        kg.set_velocity_layer_mode(&LayerVelocityMode::Automatic);

        let actual_automatic_velocity = kg
            .layers
            .clone()
            .map(|layer| -> Option<RangeInclusive<u8>> { Some(layer?.velocity) });

        assert_eq!(automatic_velocity, actual_automatic_velocity);

        kg.set_velocity_layer_mode(&LayerVelocityMode::Unison);

        let actual_unison_velocity = kg
            .layers
            .clone()
            .map(|layer| -> Option<RangeInclusive<u8>> { Some(layer?.velocity) });

        assert_eq!(unison_velocity, actual_unison_velocity);
    }
}
