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
        self.keygroups
            .sort_by(|a, b| a.layers[0].root.cmp(&b.layers[0].root));
        let kg_with_root: Vec<&mut Keygroup> = self
            .keygroups
            .iter_mut()
            .filter(|kg| kg.layers[0].root.is_some())
            .collect();

        let roots: Vec<MidiNote> = kg_with_root
            .iter()
            .map(|kg| kg.layers[0].root.unwrap())
            .collect();
        let ranges = range::build_ranges(&roots, pitch_preference);

        for (kg, range) in kg_with_root.into_iter().zip(ranges.into_iter()) {
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
        self.keygroups
            .iter()
            .all(|kg| kg.range.is_some() && kg.layers.iter().all(|layer| layer.root.is_some()))
    }

    pub fn update(&mut self, keygroups: Vec<Keygroup>, pitch_preference: f32) {
        let mut guess_ranges = false;
        if keygroups.len() != self.keygroups.len() {
            guess_ranges = true;
        } else {
            for (kg, new_kg) in self.keygroups.iter().zip(keygroups.iter()) {
                let first_layer = &kg.layers[0];
                let new_first_layer = &new_kg.layers[0];
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

#[derive(Debug, Clone)]
pub struct Keygroup {
    pub range: Option<Range>,
    pub layers: Vec<Layer>,
}

impl Keygroup {
    pub fn new(range: Range, layers: Vec<Layer>) -> Self {
        Self {
            range: Some(range),
            layers,
        }
    }

    pub fn from_file(file: String) -> Self {
        Self {
            range: None,
            layers: vec![Layer::from_file(file)],
        }
    }

    pub fn color(&self) -> Color32 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.layers[0].file.hash(&mut hasher);
        let color = RandomColor::new()
            .seed(hasher.finish())
            .luminosity(Luminosity::Light)
            .to_rgb_array();
        Color32::from_rgb(color[0], color[1], color[2])
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub file: String,
    pub root: Option<MidiNote>,
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

#[derive(PartialEq, Debug, Clone)]
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
