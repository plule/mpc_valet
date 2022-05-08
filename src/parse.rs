use music_note::{
    midi::{MidiNote, Octave},
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NoteFile {
    pub root: MidiNote,
    pub filename: String,
}

impl NoteFile {
    pub fn new(filename: String, root: MidiNote) -> Self {
        Self { filename, root }
    }
}

impl PartialOrd for NoteFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.root.partial_cmp(&other.root)
    }
}

impl Ord for NoteFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root.cmp(&other.root)
    }
}

pub fn find_samples_roots(filenames: Vec<&str>) -> Vec<NoteFile> {
    let letter_note_re =
        Regex::new(r"(?P<letter>[A-G])(?P<accidental>#?b?)(?P<octave>10|[0-9])").unwrap();

    let letter_note_results: Vec<NoteFile> = filenames
        .iter()
        .filter_map(|filename| match letter_note_re.captures(filename) {
            Some(capture) => {
                let letter = capture.name("letter").unwrap().as_str();
                let accidental = capture.name("accidental").unwrap().as_str();
                let octave = capture.name("octave").unwrap().as_str();

                let natural = match letter {
                    "A" => Natural::A,
                    "B" => Natural::B,
                    "C" => Natural::C,
                    "D" => Natural::D,
                    "E" => Natural::E,
                    "F" => Natural::F,
                    "G" => Natural::G,
                    _ => unreachable!(),
                };

                let pitch = match accidental {
                    "#" => Sharp::into_pitch(AccidentalKind::Single, natural),
                    "b" => Flat::into_pitch(AccidentalKind::Single, natural),
                    "" => Pitch::natural(natural),
                    _ => unreachable!(),
                };

                let octave = octave.parse::<i8>().unwrap();
                let octave = Octave::new_unchecked(octave);

                let midi_note = MidiNote::new(pitch, octave);
                Some(NoteFile::new(filename.to_string(), midi_note))
            }
            None => None,
        })
        .collect();

    if letter_note_results.len() == filenames.len() {
        return letter_note_results;
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_note::midi;

    #[test]
    fn parse_roots() {
        let filenames = vec![
            "MELCEL-A2.WAV",
            "MELCEL-A3.WAV",
            "MELCEL-A4.WAV",
            "MELCEL-D3.WAV",
            "MELCEL-D4.WAV",
            "MELCEL-D5.WAV",
        ];

        let notefiles = find_samples_roots(filenames);
        assert!(
            notefiles
                == vec![
                    NoteFile::new("MELCEL-A2.WAV".to_string(), midi!(A, 2)),
                    NoteFile::new("MELCEL-A3.WAV".to_string(), midi!(A, 3)),
                    NoteFile::new("MELCEL-A4.WAV".to_string(), midi!(A, 4)),
                    NoteFile::new("MELCEL-D3.WAV".to_string(), midi!(D, 3)),
                    NoteFile::new("MELCEL-D4.WAV".to_string(), midi!(D, 4)),
                    NoteFile::new("MELCEL-D5.WAV".to_string(), midi!(D, 5)),
                ]
        );
    }
}
