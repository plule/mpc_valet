use music_note::midi::MidiNote;
use xmltree::{Element, XMLNode};

use crate::process::Range;

pub struct KeyGroup {
    range: Range,
    root: MidiNote,
    sample: String,
}

impl KeyGroup {
    pub fn new(range: Range, root: MidiNote, sample: String) -> Self {
        Self {
            range,
            root,
            sample,
        }
    }
}

pub fn make_program(name: String, keygroups: Vec<KeyGroup>) -> Element {
    let reference = include_str!("Reference.xpm");
    let mut program_root = Element::parse(reference.as_bytes()).unwrap();
    let program = program_root
        .get_mut_child("MPCVObject")
        .unwrap()
        .get_mut_child("Program")
        .unwrap();

    let program_name = program.get_mut_child("ProgramName").unwrap();
    program_name.children.push(XMLNode::Text(name));

    program_root
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn make_program_test() {
        let program = make_program(
            "Hello World".to_string(),
            vec![KeyGroup::new(
                Range::new(MidiNote::from(0), MidiNote::from(127)),
                MidiNote::from(47),
                "HELLO".to_string(),
            )],
        );

        assert_eq!(
            program
                .get_child("MPCVObject")
                .unwrap()
                .get_child("Program")
                .unwrap()
                .get_child("ProgramName")
                .unwrap()
                .get_text()
                .unwrap(),
            "Hello World"
        );
    }
}
