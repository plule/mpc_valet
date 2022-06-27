#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod export;
pub mod parse;
pub mod range;
pub mod widgets;
use anyhow::Result;
use egui::Color32;
use random_color::{Luminosity, RandomColor};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    io::Write,
};

pub use app::TemplateApp;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use lazy_static::lazy_static;
use music_note::{
    midi::{MidiNote, Octave},
    Pitch,
};
use xmltree::EmitterConfig;

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(TemplateApp::new(cc))))
}

pub const OCTAVES: [Octave; 10] = [
    Octave::NEGATIVE_ONE,
    Octave::ZERO,
    Octave::ONE,
    Octave::TWO,
    Octave::THREE,
    Octave::FOUR,
    Octave::FIVE,
    Octave::SIX,
    Octave::SEVEN,
    Octave::EIGHT,
];

pub const PITCHES: [Pitch; 12] = [
    Pitch::C,
    Pitch::CSharp,
    Pitch::D,
    Pitch::DSharp,
    Pitch::E,
    Pitch::F,
    Pitch::FSharp,
    Pitch::G,
    Pitch::GSharp,
    Pitch::A,
    Pitch::ASharp,
    Pitch::B,
];

lazy_static! {
    static ref MIDI_NOTES: Vec<MidiNote> = {
        let mut notes = Vec::new();
        for octave in OCTAVES {
            for pitch in PITCHES {
                notes.push(MidiNote::new(pitch, octave));
            }
        }
        notes
    };
}

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
                                .filter(|root| root >= &range.low && root <= &range.high)
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Layer {
    pub file: String,
    pub root: Option<MidiNote>,
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
    pub fn new(file: String, root: MidiNote) -> Self {
        Self {
            file,
            root: Some(root),
        }
    }

    pub fn from_file(file: String) -> Self {
        let root = parse::parse_note(&file);
        Self { file, root }
    }

    pub fn guess_root(&mut self) {
        self.root = parse::parse_note(&self.file);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub low: MidiNote,
    pub high: MidiNote,
}

impl Range {
    pub fn new(low: MidiNote, high: MidiNote) -> Self {
        Self { low, high }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            low: MidiNote::from_byte(0),
            high: MidiNote::from_byte(127),
        }
    }
}
