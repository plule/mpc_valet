use music_note::midi::MidiNote;

use crate::parse;

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
