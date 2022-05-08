use music_note::midi::MidiNote;
use xmltree::{Element, XMLNode};

use crate::{keygroup::KeyGroup, range::Range};

trait SetChildText {
    fn set_child_text(&mut self, child: &str, text: String);
}

impl SetChildText for Element {
    fn set_child_text(&mut self, child: &str, text: String) {
        self.get_mut_child(child).unwrap().set_text(text);
    }
}

trait SetText {
    fn set_text(&mut self, text: String);
}

impl SetText for Element {
    fn set_text(&mut self, text: String) {
        self.children.clear();
        self.children.push(XMLNode::Text(text));
    }
}

pub fn make_program(name: &str, keygroups: Vec<KeyGroup>) -> Element {
    let reference = include_str!("Reference.xpm");
    let mut program_root = Element::parse(reference.as_bytes()).unwrap();
    let program = program_root.get_mut_child("Program").unwrap();

    program.set_child_text("ProgramName", name.to_string());
    program.set_child_text("KeygroupNumKeygroups", keygroups.len().to_string());

    let program_keygroups = program.get_mut_child("Instruments").unwrap();
    let reference_keygroup = program_keygroups.take_child("Instrument").unwrap();

    for (i, keygroup) in keygroups.into_iter().enumerate() {
        let mut program_keygroup = reference_keygroup.clone();
        let keygroup_number = i + 1;
        let low_note = (keygroup.range.low.into_byte() as u32) + 12;
        let high_note = (keygroup.range.high.into_byte() as u32) + 12;
        let root_note = (keygroup.root.into_byte() as u32) + 13; // off by one in the file format
        let sample_file = keygroup.file;
        let sample_name = std::path::Path::new(&sample_file)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        program_keygroup.set_child_text("LowNote", low_note.to_string());
        program_keygroup.set_child_text("HighNote", high_note.to_string());
        program_keygroup
            .attributes
            .insert("number".to_string(), keygroup_number.to_string());

        let program_layer = program_keygroup
            .get_mut_child("Layers")
            .unwrap()
            .get_mut_child("Layer")
            .unwrap();

        program_layer.set_child_text("RootNote", root_note.to_string());
        program_layer.set_child_text("SampleName", sample_name);
        program_layer.set_child_text("SampleFile", sample_file);

        program_keygroups
            .children
            .push(XMLNode::Element(program_keygroup));
    }

    program_root
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn make_program_test() {
        let program = make_program(
            "Hello World",
            vec![KeyGroup::new(
                Range::new(MidiNote::from(0), MidiNote::from(127)),
                MidiNote::from(47),
                "HELLO".to_string(),
            )],
        );

        assert_eq!(
            program
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
