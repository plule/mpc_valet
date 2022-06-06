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
    pub fn add_files(&mut self, files: Vec<String>) {
        self.keygroups
            .extend(files.into_iter().map(Keygroup::from_file));
    }

    pub fn guess_ranges(&mut self, pitch_preference: f32) {
        self.keygroups.sort();

        // keep only the keygroups with root note, and iterate in the root notes
        let (keygroups_with_root_note, root_notes): (Vec<_>, Vec<_>) = self
            .keygroups
            .iter_mut()
            .filter_map(|kg| {
                kg.first_assigned_layer()
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

    pub fn update(&mut self, keygroups: Vec<Keygroup>, pitch_preference: f32) {
        let mut guess_ranges = false;
        if keygroups.len() != self.keygroups.len() {
            guess_ranges = true;
        } else {
            for (kg, new_kg) in self.keygroups.iter().zip(keygroups.iter()) {
                let first_layer = kg.first_assigned_layer().unwrap();
                let new_first_layer = new_kg.first_assigned_layer().unwrap();
                if first_layer.file != new_first_layer.file
                    || first_layer.root != new_first_layer.root
                {
                    guess_ranges = true;
                    break;
                }
            }
        }

        self.keygroups = keygroups;

        if guess_ranges {
            self.guess_ranges(pitch_preference);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keygroup {
    pub range: Option<Range>,
    pub layers: [Option<Layer>; 4],
}

impl PartialOrd for Keygroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.first_assigned_layer()
            .partial_cmp(&other.first_assigned_layer())
    }
}

impl Ord for Keygroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first_assigned_layer()
            .cmp(&other.first_assigned_layer())
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
