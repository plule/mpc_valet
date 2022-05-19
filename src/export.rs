use anyhow::{Context, Result};
use xmltree::{Element, XMLNode};

use crate::Keygroup;

trait SetChildText {
    fn set_child_text(&mut self, child: &str, text: String) -> Result<()>;
}

impl SetChildText for Element {
    fn set_child_text(&mut self, child: &str, text: String) -> Result<()> {
        self.get_mut_child(child)
            .context(format!("No child named {}", child))?
            .set_text(text);
        Ok(())
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

pub fn make_program<'a, I>(name: &str, keygroups: I) -> Result<Element>
where
    I: IntoIterator<Item = &'a Keygroup>,
{
    let reference = include_str!("Reference.xpm");
    let mut program_root =
        Element::parse(reference.as_bytes()).context("Failed to parse the reference XPM")?;
    let program = program_root
        .get_mut_child("Program")
        .context("Failed to get the XPM root program")?;

    program.set_child_text("ProgramName", name.to_string())?;

    let program_keygroups = program
        .get_mut_child("Instruments")
        .context("Failed to get the XPM instruments")?;
    let reference_keygroup = program_keygroups
        .take_child("Instrument")
        .context("Failed to get the XPM reference instrument")?;

    let mut num_keygroups = 0;
    for (sample_file, settings) in keygroups.into_iter().filter_map(|kg| {
        kg.settings
            .as_ref()
            .map(|settings| (kg.file.to_string(), settings.clone()))
    }) {
        num_keygroups += 1;
        let mut program_keygroup = reference_keygroup.clone();
        let keygroup_number = num_keygroups;
        let range = settings.range;
        let root = settings.root;
        let low_note = range.low.into_byte() as u32;
        let high_note = range.high.into_byte() as u32;
        let root_note = (root.into_byte() as u32) + 1; // off by one in the file format
        let sample_name = std::path::Path::new(&sample_file)
            .file_stem()
            .context("Failed to find the sample base name")?
            .to_str()
            .context("The sample does not have a valid base name")?
            .to_string();

        program_keygroup.set_child_text("LowNote", low_note.to_string())?;
        program_keygroup.set_child_text("HighNote", high_note.to_string())?;
        program_keygroup
            .attributes
            .insert("number".to_string(), keygroup_number.to_string());

        let program_layer = program_keygroup
            .get_mut_child("Layers")
            .context("Failed to find the XPM reference program layers")?
            .get_mut_child("Layer")
            .context("Failed to get the XPM reference program layer")?;

        program_layer.set_child_text("RootNote", root_note.to_string())?;
        program_layer.set_child_text("SampleName", sample_name)?;
        program_layer.set_child_text("SampleFile", sample_file)?;

        program_keygroups
            .children
            .push(XMLNode::Element(program_keygroup));
    }

    program.set_child_text("KeygroupNumKeygroups", num_keygroups.to_string())?;

    Ok(program_root)
}

#[cfg(test)]
mod tests {
    use music_note::midi::MidiNote;

    use crate::Range;

    pub use super::*;

    #[test]
    fn make_program_test() {
        let program = make_program(
            "Hello World",
            &vec![Keygroup::new(
                Range::new(MidiNote::from(0), MidiNote::from(127)),
                MidiNote::from(47),
                "HELLO".to_string(),
            )],
        )
        .expect("Could not make the program at all");

        assert_eq!(
            program
                .get_child("Program")
                .expect("no program root")
                .get_child("ProgramName")
                .expect("no program name")
                .get_text()
                .expect("no program text"),
            "Hello World"
        );
    }
}
