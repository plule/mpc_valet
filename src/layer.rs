use std::ops::RangeInclusive;

use music_note::midi::MidiNote;

use crate::PartialFromStr;

/// MPC keygroup layer.
///
/// Each layer is an assigned file with a root note and a velocity
/// range where it is active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    /// Sample file
    pub file: String,

    /// Root note
    ///
    /// None if failed to be deduced from the sample name.
    pub root: Option<MidiNote>,

    /// Velocity range where this layer should be active.
    pub velocity: RangeInclusive<u8>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            file: Default::default(),
            root: Default::default(),
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
            root: Some(root),
            velocity,
        }
    }

    /// Build a layer from a file and guess its root note.
    pub fn from_file(file: String) -> Self {
        let mut layer = Self {
            file,
            ..Default::default()
        };
        layer.guess_root();
        layer
    }

    /// Guess the root note of a layer.
    pub fn guess_root(&mut self) {
        self.root = MidiNote::partial_from_str(&self.file).ok().map(|r| r.value);
    }
}
