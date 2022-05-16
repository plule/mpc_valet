#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod export;
pub mod keyboard;
pub mod parse;
pub mod range;
use anyhow::Result;
use std::io::Write;

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

    pub fn guess_roots(&mut self) {
        let filenames: Vec<&str> = self.keygroups.iter().map(|kg| kg.file.as_str()).collect();
        let roots = parse::find_best_candidate(filenames.clone());
        for (kg, root) in self.keygroups.iter_mut().zip(roots.into_iter()) {
            if let Some(root) = root {
                kg.settings = Some(KeygroupSettings::new(root, Range::default()));
            }
        }
    }

    pub fn guess_ranges(&mut self, pitch_preference: f32) {
        self.keygroups.sort_by(|a, b| a.settings.cmp(&b.settings));
        let kg_settings: Vec<&mut KeygroupSettings> = self
            .keygroups
            .iter_mut()
            .filter_map(|kg| kg.settings.as_mut())
            .collect();
        let roots: Vec<MidiNote> = kg_settings.iter().map(|kg| kg.root).collect();
        let ranges = range::build_ranges(&roots, pitch_preference);
        for (kg, range) in kg_settings.into_iter().zip(ranges.into_iter()) {
            kg.range = range;
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
        self.keygroups.iter().any(|kg| kg.settings.is_some())
    }
}

#[derive(Debug)]
pub struct Keygroup {
    pub file: String,
    pub settings: Option<KeygroupSettings>,
}

impl Keygroup {
    pub fn new(range: Range, root: MidiNote, file: String) -> Self {
        Self {
            file,
            settings: Some(KeygroupSettings::new(root, range)),
        }
    }

    pub fn from_file(file: String) -> Self {
        Self {
            file,
            settings: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeygroupSettings {
    pub root: MidiNote,
    pub range: Range,
}

impl Eq for KeygroupSettings {}

impl Ord for KeygroupSettings {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.cmp(&other.root)
    }
}

impl PartialOrd for KeygroupSettings {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.root.partial_cmp(&other.root)
    }
}

impl KeygroupSettings {
    pub fn new(root: MidiNote, range: Range) -> Self {
        Self { root, range }
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
