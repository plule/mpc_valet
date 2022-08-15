use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use xmltree::EmitterConfig;

use crate::utils::{build_ranges, make_program};

use super::{Keygroup, Layer, LayerFile, LayerVelocityMode};

/// A keygroup program is an instrument based on samples.
///
/// It's split into multiple note range, each one in a Keygroup.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeygroupProgram {
    /// Name of the keygroup program.
    pub name: String,

    /// Keygroups making this program.
    pub keygroups: Vec<Keygroup>,
}

impl KeygroupProgram {
    pub fn insert_layer_files(&mut self, files: Vec<LayerFile>) {
        for file in files.into_iter() {
            self.insert_layer_file(file);
        }
    }

    pub fn insert_layer_file(&mut self, file: LayerFile) {
        let new_layer = Some(Layer::new(file.file, file.root, 0..=127));
        // Look for a keygroup with this layer file
        if let Some(kg) = self.keygroups.iter_mut().find(|kg| {
            kg.layers
                .iter()
                .any(|layer| layer.iter().any(|layer| layer.root == file.root))
        }) {
            kg.layers[file.layer] = new_layer;
        } else {
            let mut layers = [None, None, None, None];
            layers[file.layer] = new_layer;
            self.keygroups.push(Keygroup::new(0..=127, layers));
        }
    }

    /// Sort the keygroups of the program.
    pub fn sort_keygroups(&mut self) {
        self.keygroups.sort();
    }

    /// Based on the first layer, guess the ranges of each keygroup.
    ///
    /// The pitch preference should be between 0 and 1 and is used to choose between
    /// pitching down or pitching up the samples. 0.5 means that each root note
    /// will be at the "center" of its keygroup.
    pub fn guess_ranges(&mut self, pitch_preference: f32) {
        // keep only the keygroups with root note, and iterate in the root notes
        let (keygroups_with_root_note, root_notes): (Vec<_>, Vec<_>) = self
            .keygroups
            .iter_mut()
            .filter_map(|kg| {
                kg.layers[0]
                    .as_ref()
                    .map(|layer| layer.root)
                    .map(|root_note| (kg, root_note))
            })
            .unzip();

        // guess the ranges from the root notes
        let ranges = build_ranges(&root_notes, pitch_preference);

        // assign the ranges to the keygroups with root notes
        for (kg, range) in keygroups_with_root_note.into_iter().zip(ranges.into_iter()) {
            kg.range = range;
        }
    }

    pub fn export<W: Write>(&self, w: W) -> Result<()> {
        let program = make_program(&self.name, &self.keygroups)?;
        let mut cfg = EmitterConfig::new();
        cfg.perform_indent = true;

        program.write_with_config(w, cfg)?;
        Ok(())
    }

    pub fn layer_count(&self) -> usize {
        self.keygroups
            .iter()
            .map(|kg| kg.layer_count())
            .max()
            .unwrap_or_default()
    }

    pub fn set_velocity_layer_mode(&mut self, mode: &LayerVelocityMode) {
        for keygroup in self.keygroups.iter_mut() {
            keygroup.set_velocity_layer_mode(mode);
        }
    }
}
