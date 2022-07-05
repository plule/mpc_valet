use std::fmt::Display;

/// Velocity range assignment mode.
#[derive(PartialEq, Clone, Copy)]
pub enum LayerVelocityMode {
    /// Assign non overlapping ranges to each layer.
    Spread,

    /// Set the full range to all the layers.
    Overlapping,
}

impl Default for LayerVelocityMode {
    fn default() -> Self {
        Self::Spread
    }
}

impl Display for LayerVelocityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerVelocityMode::Spread => write!(f, "Spread"),
            LayerVelocityMode::Overlapping => write!(f, "Overlapping"),
        }
    }
}
