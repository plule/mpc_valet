use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

/// MPC keygroup layer.
///
/// Each layer is an assigned file with a root note and a velocity
/// range where it is active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer {
    /// Sample file
    pub file: String,

    /// Root note
    pub root: u8,

    /// Velocity range where this layer should be active.
    pub velocity: RangeInclusive<u8>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            file: Default::default(),
            root: 0,
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
    pub fn new(file: String, root: u8, velocity: RangeInclusive<u8>) -> Self {
        Self {
            file,
            root,
            velocity,
        }
    }
}
