use std::{fmt::Display, ops::RangeInclusive};

use serde::{Deserialize, Serialize};

/// Velocity range assignment mode.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LayerVelocityMode {
    /// Assign non overlapping ranges to each layer.
    Automatic,

    /// Set the full range to all the layers.
    Unison,

    /// Manually set the range of each layer
    Manual([RangeInclusive<u8>; 4]),
}

impl Default for LayerVelocityMode {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(PartialEq, Clone)]
pub enum LayerVelocityModeSelection {
    Automatic,
    Unison,
    Manual,
}

impl Display for LayerVelocityModeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerVelocityModeSelection::Automatic => write!(f, "Automatic"),
            LayerVelocityModeSelection::Unison => write!(f, "Unison"),
            LayerVelocityModeSelection::Manual => write!(f, "Manual"),
        }
    }
}

impl From<LayerVelocityMode> for LayerVelocityModeSelection {
    fn from(mode: LayerVelocityMode) -> Self {
        match mode {
            LayerVelocityMode::Automatic => LayerVelocityModeSelection::Automatic,
            LayerVelocityMode::Unison => LayerVelocityModeSelection::Unison,
            LayerVelocityMode::Manual(_) => LayerVelocityModeSelection::Manual,
        }
    }
}

impl From<LayerVelocityModeSelection> for LayerVelocityMode {
    fn from(mode: LayerVelocityModeSelection) -> Self {
        match mode {
            LayerVelocityModeSelection::Automatic => LayerVelocityMode::Automatic,
            LayerVelocityModeSelection::Unison => LayerVelocityMode::Unison,
            LayerVelocityModeSelection::Manual => {
                LayerVelocityMode::Manual([0..=127, 0..=127, 0..=127, 0..=127])
            }
        }
    }
}
