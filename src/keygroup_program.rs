use anyhow::Result;
use std::{collections::HashSet, io::Write};
use xmltree::EmitterConfig;

use crate::{export, range, Keygroup, Layer};

#[derive(Debug)]
pub struct KeygroupProgram {
    pub name: String,
    pub keygroups: Vec<Keygroup>,
}

impl Default for KeygroupProgram {
    fn default() -> Self {
        Self {
            name: "My Keygroup Program".to_string(),
            keygroups: Default::default(),
        }
    }
}

impl KeygroupProgram {
    pub fn add_files(&mut self, layer: usize, files: HashSet<String>) {
        let existing_files: HashSet<String> = HashSet::from_iter(
            self.keygroups
                .iter()
                .filter_map(|kg| kg.layers[layer].as_ref())
                .map(|l| l.file.clone()),
        );

        let files: Vec<String> = files
            .into_iter()
            .filter(|file| !existing_files.contains(file))
            .collect();

        for file in files {
            let keygroup = if let Some(kg) = self
                .keygroups
                .iter_mut()
                .find(|kg| kg.layers[layer].is_none())
            {
                kg
            } else {
                self.keygroups.push(Keygroup::default());
                self.keygroups.last_mut().unwrap()
            };

            let layer = keygroup.layers[layer].get_or_insert(Layer::default());
            layer.file = file.to_string();
            layer.guess_root();
        }
    }

    fn sort_keygroups(&mut self) {
        self.keygroups.sort();
    }

    pub fn sort_layer(&mut self, layer_index: usize) {
        match layer_index {
            0 => {
                self.sort_keygroups();

                // Resort the other layers
                for layer_index in 1..4 {
                    self.sort_layer(layer_index);
                }
            }
            _ => {
                // Store the layer
                let mut layers: Vec<Option<Layer>> = self
                    .keygroups
                    .iter()
                    .map(|kg| kg.layers[layer_index].clone())
                    .collect();

                // Assign matching layers
                for kg in self.keygroups.iter_mut() {
                    kg.layers[layer_index] = None;

                    if let Some(range) = &kg.range {
                        // Find the index of a layer with ranges corresponding to the root of this sample
                        let matching_layer = layers.iter().position(|layer| {
                            layer
                                .as_ref()
                                .and_then(|layer| layer.root)
                                .filter(|root| range.contains(root))
                                .is_some()
                        });

                        // If found, assign it and remove it from the layer list being sorted
                        if let Some(matching_position) = matching_layer {
                            kg.layers[layer_index] = layers[matching_position].clone();
                            layers.remove(matching_position);
                        }
                    }
                }

                // Assign remaining layers
                for (unassigned_layer, kg) in layers.iter().zip(
                    self.keygroups
                        .iter_mut()
                        .filter(|kg| kg.layers[layer_index].is_none()),
                ) {
                    kg.layers[layer_index] = unassigned_layer.clone().clone();
                }
            }
        }
    }

    pub fn guess_ranges(&mut self, pitch_preference: f32) {
        // keep only the keygroups with root note, and iterate in the root notes
        let (keygroups_with_root_note, root_notes): (Vec<_>, Vec<_>) = self
            .keygroups
            .iter_mut()
            .filter_map(|kg| {
                kg.layers[0]
                    .as_ref()
                    .and_then(|layer| layer.root)
                    .map(|root_note| (kg, root_note))
            })
            .unzip();

        // guess the ranges from the root notes
        let ranges = range::build_ranges(&root_notes, pitch_preference);

        // assign the ranges to the keygroups with root notes
        for (kg, range) in keygroups_with_root_note.into_iter().zip(ranges.into_iter()) {
            kg.range = Some(range);
        }
    }

    pub fn export<W: Write>(&self, w: W) -> Result<()> {
        let program = export::make_program(&self.name, &self.keygroups)?;
        let mut cfg = EmitterConfig::new();
        cfg.perform_indent = true;

        program.write_with_config(w, cfg)?;
        Ok(())
    }

    pub fn can_export(&self) -> bool {
        self.keygroups.iter().all(|kg| kg.range.is_some())
    }

    pub fn update(&mut self, layer: usize, keygroups: Vec<Keygroup>, pitch_preference: f32) {
        let default_layer = Layer::default();

        // Remove empty layers from the input
        let keygroups: Vec<Keygroup> = keygroups
            .into_iter()
            .filter(|kg| kg.layers.iter().any(|layer| layer.is_some()))
            .collect();

        // If there is any file change or root note change, guess again the ranges
        let guess_ranges = (keygroups.len() != self.keygroups.len())
            || self
                .keygroups
                .iter()
                .zip(keygroups.iter())
                .any(|(kg, new_kg)| {
                    let previous_layer = kg.layers[layer].as_ref().unwrap_or(&default_layer);
                    let new_layer = new_kg.layers[layer].as_ref().unwrap_or(&default_layer);
                    previous_layer.file != new_layer.file || previous_layer.root != new_layer.root
                });

        self.keygroups = keygroups;

        if guess_ranges {
            self.guess_ranges(pitch_preference);
        }
    }
}
