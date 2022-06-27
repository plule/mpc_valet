use std::ops::RangeInclusive;

use music_note::midi::MidiNote;

use crate::parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub file: String,
    pub root: Option<MidiNote>,
    pub velocity: RangeInclusive<u8>,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            file: Default::default(),
            root: Default::default(),
            velocity: 0..=127,
        }
    }
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
    pub fn new(file: String, root: MidiNote, velocity: RangeInclusive<u8>) -> Self {
        Self {
            file,
            root: Some(root),
            velocity,
        }
    }

    pub fn from_file(file: String) -> Self {
        let root = parse::parse_note(&file);
        Self {
            file,
            root,
            ..Default::default()
        }
    }

    pub fn guess_root(&mut self) {
        self.root = parse::parse_note(&self.file);
    }
}
