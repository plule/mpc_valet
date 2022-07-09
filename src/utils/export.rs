use anyhow::{bail, Context, Result};
use xmltree::{Element, XMLNode};

use crate::model::Keygroup;

/// Trait for the ability to set a child member.
///
/// For example: `set_child_text("Hello", "World")` results in:
/// <Hello>World</Hello>.
///
/// This construction is used a lot in MPC's XMLs.
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

/// Make this element content just text data.
trait SetText {
    fn set_text(&mut self, text: String);
}

impl SetText for Element {
    fn set_text(&mut self, text: String) {
        self.children.clear();
        self.children.push(XMLNode::Text(text));
    }
}

/// Export a keygroup program to an XML ready to be saved as .xpm.
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
    for keygroup in keygroups.into_iter() {
        num_keygroups += 1;
        let mut program_keygroup = reference_keygroup.clone();
        let keygroup_number = num_keygroups;
        let range = &keygroup.range;

        let low_note = *range.start() as u32;
        let high_note = *range.end() as u32;

        program_keygroup.set_child_text("LowNote", low_note.to_string())?;
        program_keygroup.set_child_text("HighNote", high_note.to_string())?;
        program_keygroup
            .attributes
            .insert("number".to_string(), keygroup_number.to_string());

        let program_layers = program_keygroup
            .get_mut_child("Layers")
            .context("Failed to find the XPM reference program layers")?;

        let program_layers: Vec<&mut Element> = program_layers
            .children
            .iter_mut()
            .filter_map(|c| match c {
                XMLNode::Element(e) => Some(e),
                _ => None,
            })
            .filter(|e| e.name == "Layer")
            .collect();

        if program_layers.len() != 4 {
            bail!("The reference program does not contain 4 layers");
        }

        for (layer, program_layer) in keygroup.layers.iter().zip(program_layers) {
            if let Some(layer) = layer {
                let sample_file = layer.file.clone();

                let sample_name = std::path::Path::new(&sample_file)
                    .file_stem()
                    .context("Failed to find the sample base name")?
                    .to_str()
                    .context("The sample does not have a valid base name")?
                    .to_string();

                let velocity_start = layer.velocity.start().to_string();
                let velocity_end = layer.velocity.end().to_string();

                program_layer.set_child_text("SampleName", sample_name)?;
                program_layer.set_child_text("SampleFile", sample_file)?;
                program_layer.set_child_text("VelStart", velocity_start)?;
                program_layer.set_child_text("VelEnd", velocity_end)?;
                let root_note = (layer.root as u32) + 1; // off by one in the file format
                program_layer.set_child_text("RootNote", root_note.to_string())?;
            }
        }

        program_keygroups
            .children
            .push(XMLNode::Element(program_keygroup));
    }

    program.set_child_text("KeygroupNumKeygroups", num_keygroups.to_string())?;

    Ok(program_root)
}

#[cfg(test)]
mod tests {
    use crate::model::Layer;

    pub use super::*;

    #[test]
    fn make_program_test() {
        let program = make_program(
            "Hello World",
            &vec![Keygroup::new(
                0..=127,
                [
                    Some(Layer::new("HELLO.wav".to_string(), 47, 25..=56)),
                    None,
                    None,
                    None,
                ],
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

        let layer = program
            .get_child("Program")
            .expect("no program root")
            .get_child("Instruments")
            .expect("no instrument list")
            .get_child("Instrument")
            .expect("no instrument in the list")
            .get_child("Layers")
            .expect("no layer list")
            .get_child("Layer")
            .expect("no layer in the list");

        assert_eq!(
            layer
                .get_child("SampleFile")
                .expect("no sample file")
                .get_text()
                .unwrap(),
            "HELLO.wav"
        );

        assert_eq!(
            layer
                .get_child("VelStart")
                .expect("no velocity start")
                .get_text()
                .unwrap(),
            "25"
        );

        assert_eq!(
            layer
                .get_child("VelEnd")
                .expect("no velocity end")
                .get_text()
                .unwrap(),
            "56"
        );
    }
}
