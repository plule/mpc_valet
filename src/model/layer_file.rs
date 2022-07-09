use serde::{Deserialize, Serialize};

use super::SampleFile;

/// A sample file with a root note, assigned to a layer
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct LayerFile {
    /// Sample file (.wav)
    pub file: String,

    /// Root note of the original file.
    pub root: u8,

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
