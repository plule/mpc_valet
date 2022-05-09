#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod export;
pub mod keygroup;
pub mod parse;
pub mod range;

use std::io::Write;

pub use app::TemplateApp;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use music_note::midi::MidiNote;
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
            kg.root = root;
        }

        self.keygroups.sort_by_key(|kg| kg.root);
    }

    pub fn guess_ranges(&mut self) {
        let kg_with_root: Vec<&mut Keygroup> = self
            .keygroups
            .iter_mut()
            .filter(|kg| kg.root.is_some())
            .collect();
        let roots: Vec<MidiNote> = kg_with_root.iter().map(|kg| kg.root.unwrap()).collect();
        let ranges = range::build_ranges(&roots);
        for (kg, range) in kg_with_root.into_iter().zip(ranges.into_iter()) {
            kg.range = range;
        }
    }

    pub fn export<W: Write>(&self, w: W) {
        let program = export::make_program(&self.name, &self.keygroups);
        let mut cfg = EmitterConfig::new();
        cfg.perform_indent = true;

        program.write_with_config(w, cfg).unwrap();
    }
}

#[derive(Debug)]
pub struct Keygroup {
    pub range: Range,
    pub root: Option<MidiNote>,
    pub file: String,
}

impl Keygroup {
    pub fn new(range: Range, root: MidiNote, file: String) -> Self {
        Self {
            range,
            root: Some(root),
            file,
        }
    }

    pub fn from_file(file: String) -> Self {
        Self {
            file,
            root: None,
            range: Range::default(),
        }
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
