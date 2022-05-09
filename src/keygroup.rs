use crate::parse::find_best_candidate;
use crate::range::build_ranges;
use music_note::midi::MidiNote;

use crate::Keygroup;

struct Sample {
    file: String,
    root: MidiNote,
}

impl Sample {
    fn new(file: String, root: MidiNote) -> Self {
        Self { file, root }
    }
}

pub fn make_keygroups<'a, I>(filenames: I) -> Vec<Keygroup>
where
    I: IntoIterator<Item = &'a str>,
{
    let filenames: Vec<&str> = filenames.into_iter().collect();
    let roots = find_best_candidate(filenames.clone());
    let mut samples: Vec<Sample> = filenames
        .into_iter()
        .zip(roots.into_iter())
        .map(|(name, root)| Sample::new(name.to_string(), root.unwrap()))
        .collect();
    samples.sort_by_key(|s| s.root);
    let ranges = build_ranges(samples.iter().map(|s| &s.root));
    let keygroups = samples
        .into_iter()
        .zip(ranges.into_iter())
        .map(|(sample, range)| Keygroup::new(range, sample.root, sample.file))
        .collect();
    keygroups
}
