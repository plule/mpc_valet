use lazy_static::lazy_static;
use music_note::{
    midi::{MidiNote, Octave},
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

pub fn parse_letter_notation(filename: &str) -> Option<MidiNote> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<letter>[A-G])(?P<accidental>#?b?)(?P<octave>10|-?[0-9])").unwrap();
    }

    match RE.captures(filename) {
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
    }
}

/// Parse a list of file names and return their midi notes.
pub fn find_best_candidate<'a, I>(filenames: I) -> Vec<Option<MidiNote>>
where
    I: IntoIterator<Item = &'a str>,
{
    filenames.into_iter().map(parse_letter_notation).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_note::midi;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["MELCEL-A2.WAV", "MELCEL-A3.WAV", "MELCEL-A4.WAV", "MELCEL-D3.WAV", "MELCEL-D4.WAV", "MELCEL-D5.WAV"],
        vec![Some(midi!(A, 2)), Some(midi!(A, 3)), Some(midi!(A, 4)), Some(midi!(D, 3)), Some(midi!(D, 4)), Some(midi!(D, 5))],
    )]
    fn parse_roots(#[case] input: Vec<&str>, #[case] expected: Vec<Option<MidiNote>>) {
        let midi_notes = find_best_candidate(input);
        assert_eq!(midi_notes, expected);
    }

    #[rstest]
    #[case("MELCEL-A2.WAV", midi!(A,2))]
    #[case("MELCEL-A-1.WAV", midi!(A,-1))]
    #[case("MELCEL-D0.WAV", midi!(D,0))]
    #[case("MELCEL-F4.WAV", midi!(F,4))]
    fn parse_letter_notation_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(parse_letter_notation(input), Some(expected));
    }
}
