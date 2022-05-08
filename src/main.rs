use export::make_program;
use std::env;
use std::fs::File;
use std::path::Path;
use xmltree::EmitterConfig;

use crate::keygroup::make_keygroups;

mod export;
mod keygroup;
mod parse;
mod range;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        cli(&args[1]);
    }
}

fn cli(dir: &str) {
    let program_name = Path::new(dir).file_name().unwrap().to_str().unwrap();
    println!("Processing {}", dir);
    let files = glob::glob(format!("{}/*.WAV", dir).as_str()).unwrap();
    let filenames: Vec<String> = files
        .map(|p| p.unwrap().to_str().unwrap().to_string())
        .collect();
    let keygroups = make_keygroups(filenames);
    let program = make_program(program_name, keygroups);
    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;
    program
        .write_with_config(
            File::create(format!("{}/{}.xpm", dir, program_name)).unwrap(),
            cfg,
        )
        .unwrap();
}
