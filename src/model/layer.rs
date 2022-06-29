use std::ops::RangeInclusive;

use music_note::midi::MidiNote;

/// MPC keygroup layer.
///
/// Each layer is an assigned file with a root note and a velocity
/// range where it is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    /// Sample file
    pub file: String,

    /// Root note
    pub root: MidiNote,

    /// Velocity range where this layer should be active.
    pub velocity: RangeInclusive<u8>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            file: Default::default(),
            root: MidiNote::from_byte(0),
            velocity: 0..=127,
        }
    }
}

impl Ord for Layer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.cmp(&other.root)
    }
}

impl PartialOrd for Layer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.root.partial_cmp(&other.root)
    }
}

impl Layer {
    pub fn new(file: String, root: MidiNote, velocity: RangeInclusive<u8>) -> Self {
        Self {
            file,
            root,
            velocity,
        }
    }
}
