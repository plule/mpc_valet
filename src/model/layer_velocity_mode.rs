use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Velocity range assignment mode.
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum LayerVelocityMode {
    /// Assign non overlapping ranges to each layer.
    Automatic,

    /// Set the full range to all the layers.
    Unison,
}

impl Default for LayerVelocityMode {
    fn default() -> Self {
        Self::Automatic
    }
}

impl Display for LayerVelocityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerVelocityMode::Automatic => write!(f, "Automatic"),
            LayerVelocityMode::Unison => write!(f, "Unison"),
        }
    }
}
