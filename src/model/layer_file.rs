use music_note::midi::MidiNote;

use super::SampleFile;

/// A sample file with a root note, assigned to a layer
#[derive(PartialEq, Clone)]
pub struct LayerFile {
    /// Sample file (.wav)
    pub file: String,

    /// Root note of the original file.
    pub root: MidiNote,

    /// Destination layer
    pub layer: usize,
}

impl LayerFile {
    pub fn from_sample_file(file: SampleFile, layer: usize) -> Self {
        Self {
            file: file.file,
            root: file.root,
            layer,
        }
    }
}

impl From<SampleFile> for LayerFile {
    fn from(file: SampleFile) -> Self {
        Self {
            file: file.file,
            root: file.root,
            layer: 0,
        }
    }
}
