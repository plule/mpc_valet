use export::{make_program, KeyGroup};
use music_note::midi::MidiNote;
use parse::find_samples_roots;
use process::build_ranges;
use std::env;
use std::fs::File;
use std::path::Path;
use xmltree::EmitterConfig;

mod export;
mod parse;
mod process;

struct Sample {
    file: String,
    root: MidiNote,
}

impl Sample {
    fn new(file: String, root: MidiNote) -> Self {
        Self { file, root }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args[1].to_string();
    let programname = Path::new(dir.as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    println!("Processing {}", dir);
    let files = glob::glob(format!("{}/*.WAV", args[1]).as_str()).unwrap();

    let filenames: Vec<String> = files
        .map(|p| p.unwrap().to_str().unwrap().to_string())
        .collect();

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

    let program = make_program(programname, keygroups);

    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;

    program
        .write_with_config(
            File::create(format!("{}/{}.xpm", dir, programname)).unwrap(),
            cfg,
        )
        .unwrap();
}
