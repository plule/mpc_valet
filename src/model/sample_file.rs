use music_note::midi::MidiNote;
use pomsky_macro::pomsky;

use music_note::{
    midi::Octave,
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// A sample file with a root note.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct SampleFile {
    /// Sample file (.wav)
    pub file: String,

    /// Root note
    pub root: u8,
}

impl From<String> for SampleFile {
    fn from(value: String) -> Self {
        let note = parse_letter_notation(&value).or_else(|| parse_number_notation(&value));
        if let Some(note) = note {
            return SampleFile {
                file: value,
                root: note.into_byte(),
            };
        }
        SampleFile {
            file: value,
            // TODO report
            root: 0,
        }
    }
}

/// Try parsing a file with a number midi notation (0-127)
fn parse_number_notation(filename: &str) -> Option<MidiNote> {
    const REGEX: &str = pomsky!(
        "0"* // possible leading zeroes
        :value(range "0"-"127")
    );
    lazy_static! {
        static ref RE: Regex = Regex::new(REGEX).expect("BUG: Invalid number notation regex");
    }

    let capture = RE.captures(filename)?;

    let number = capture.name("value")?.as_str().parse::<u8>().ok()?;

    Some(number.into())
}

/// Try parsing a file with a letter notation (A2)
fn parse_letter_notation(filename: &str) -> Option<MidiNote> {
    const REGEX: &str = pomsky!(
        // Do not allow a letter just before the natural letter
        // It's likely an actual word
        (Start | !["A"-"Z" "a"-"z"])
        // Natural note
        :natural(["A"-"G" "a"-"g"])
        // Optional accidental
        :accidental(""|"#"|"b")
        // Octave value
        :octave("-1" | range "0"-"8")
    );
    lazy_static! {
        static ref RE: Regex = Regex::new(REGEX).expect("BUG: Invalid letter notation regex");
    }

    let capture = RE.captures(filename)?;

    let natural = capture
        .name("natural")
        .expect("BUG: Regex did not have the letter capture")
        .as_str();
    let accidental = capture
        .name("accidental")
        .expect("BUG: Regex did not have the accidental capture")
        .as_str();
    let octave = capture
        .name("octave")
        .expect("BUG: Regex did not have the octave capture")
        .as_str();

    let natural = match natural {
        "A" | "a" => Some(Natural::A),
        "B" | "b" => Some(Natural::B),
        "C" | "c" => Some(Natural::C),
        "D" | "d" => Some(Natural::D),
        "E" | "e" => Some(Natural::E),
        "F" | "f" => Some(Natural::F),
        "G" | "g" => Some(Natural::G),
        _ => None,
    }?;

    let pitch = match accidental {
        "#" => Sharp::into_pitch(AccidentalKind::Single, natural),
        "b" => Flat::into_pitch(AccidentalKind::Single, natural),
        "" => Pitch::natural(natural),
        _ => unreachable!(),
    };

    let octave = match octave.parse::<i8>().ok()? {
        -1 => Some(Octave::NEGATIVE_ONE),
        0 => Some(Octave::ZERO),
        1 => Some(Octave::ONE),
        2 => Some(Octave::TWO),
        3 => Some(Octave::THREE),
        4 => Some(Octave::FOUR),
        5 => Some(Octave::FIVE),
        6 => Some(Octave::SIX),
        7 => Some(Octave::SEVEN),
        8 => Some(Octave::EIGHT),
        _ => None,
    }?;

    Some(MidiNote::new(pitch, octave))
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_note::midi;
    use rstest::rstest;

    #[rstest]
    #[case("A2.wav", midi!(A,2))]
    #[case("MELCEL-A2.WAV", midi!(A,2))]
    #[case("MELCEL-A-1.WAV", midi!(A,-1))]
    #[case("MELCEL-D0.WAV", midi!(D,0))]
    #[case("MELCEL-Db0.WAV", midi!(CSharp,0))]
    #[case("MELCEL-F4.WAV", midi!(F,4))]
    #[case("de_1_d#5.wav", midi!(DSharp,5))]
    fn parse_letter_notation_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(parse_letter_notation(input).unwrap(), expected);
    }

    #[rstest]
    #[case("THMB40.wav", MidiNote::from(40))]
    #[case("THMB43.wav", MidiNote::from(43))]
    #[case("THMB48.wav", MidiNote::from(48))]
    fn parse_number_notation_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(parse_number_notation(input).unwrap(), expected);
    }

    #[rstest]
    #[case("A2.wav", midi!(A,2))]
    #[case("MELCEL-A2.WAV", midi!(A,2))]
    #[case("MELCEL-A-1.WAV", midi!(A,-1))]
    #[case("MELCEL-D0.WAV", midi!(D,0))]
    #[case("MELCEL-F4.WAV", midi!(F,4))]
    #[case("THMB-40.wav", MidiNote::from(40))]
    #[case("THMB-43.wav", MidiNote::from(43))]
    #[case("THMB-48.wav", MidiNote::from(48))]
    #[case("THMB40.wav", MidiNote::from(40))]
    #[case("THMB43.wav", MidiNote::from(43))]
    #[case("THMB48.wav", MidiNote::from(48))]
    #[case("THMB048.wav", MidiNote::from(48))]
    fn parse_note_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(
            SampleFile::from(input.to_string()).root,
            expected.into_byte()
        );
    }
}
