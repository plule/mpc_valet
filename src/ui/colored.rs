use std::hash::{Hash, Hasher};

use egui::Color32;
use random_color::{Luminosity, RandomColor};

use crate::Keygroup;

pub trait Colored {
    fn color(&self) -> Color32;
}

impl Colored for Keygroup {
    /// Get a random stable color for this layer.
    fn color(&self) -> Color32 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let layer = self.first_assigned_layer().cloned().unwrap_or_default();

        layer.file.hash(&mut hasher);
        let color = RandomColor::new()
            .seed(hasher.finish())
            .luminosity(Luminosity::Light)
            .to_rgb_array();
        Color32::from_rgb(color[0], color[1], color[2])
    }
}
