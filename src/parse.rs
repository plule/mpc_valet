use music_note::{
    midi::{MidiNote, Octave},
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

/// Parse a list of file names and return their midi notes.
pub fn find_samples_roots(filenames: Vec<&str>) -> Vec<MidiNote> {
    let letter_note_re =
        Regex::new(r"(?P<letter>[A-G])(?P<accidental>#?b?)(?P<octave>10|[0-9])").unwrap();

    let midi_notes: Vec<MidiNote> = filenames
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

                Some(MidiNote::new(pitch, octave))
            }
            None => None,
        })
        .collect();

    if midi_notes.len() == filenames.len() {
        return midi_notes;
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_note::midi;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["MELCEL-A2.WAV", "MELCEL-A3.WAV", "MELCEL-A4.WAV", "MELCEL-D3.WAV", "MELCEL-D4.WAV", "MELCEL-D5.WAV"],
        vec![midi!(A, 2), midi!(A, 3), midi!(A, 4), midi!(D, 3), midi!(D, 4), midi!(D, 5)],
    )]
    fn parse_roots(#[case] input: Vec<&str>, #[case] expected: Vec<MidiNote>) {
        let midi_notes = find_samples_roots(input);
        assert_eq!(midi_notes, expected);
    }
}
