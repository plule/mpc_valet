use crate::parse::find_samples_roots;
use crate::range::build_ranges;
use music_note::midi::MidiNote;

use crate::range::Range;

struct Sample {
    file: String,
    root: MidiNote,
}

impl Sample {
    fn new(file: String, root: MidiNote) -> Self {
        Self { file, root }
    }
}

#[derive(Debug)]
pub struct KeyGroup {
    pub range: Range,
    pub root: MidiNote,
    pub file: String,
}

impl KeyGroup {
    pub fn new(range: Range, root: MidiNote, file: String) -> Self {
        Self { range, root, file }
    }
}

pub fn make_keygroups(filenames: Vec<String>) -> Vec<KeyGroup> {
    let roots = find_samples_roots(&filenames);
    let mut samples: Vec<Sample> = filenames
        .into_iter()
        .zip(roots.into_iter())
        .map(|(name, root)| Sample::new(name, root))
        .collect();
    samples.sort_by_key(|s| s.root);
    let ranges = build_ranges(samples.iter().map(|s| &s.root));
    let keygroups = samples
        .into_iter()
        .zip(ranges.into_iter())
        .map(|(sample, range)| KeyGroup::new(range, sample.root, sample.file))
        .collect();
    keygroups
}
