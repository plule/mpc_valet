use std::hash::{Hash, Hasher};

use egui::Color32;
use random_color::{Luminosity, RandomColor};

use crate::{Layer, Range};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Keygroup {
    pub range: Option<Range>,
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
    pub fn new(range: Range, layers: [Option<Layer>; 4]) -> Self {
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
}
