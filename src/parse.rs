///! Note parsing module
use music_note::{
    midi::{MidiNote, Octave},
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

use lazy_static::lazy_static;

/// Parsed value with additional suffix and prefix data.
#[derive(Debug)]
pub struct Parsed<'a, T> {
    pub value: T,
    pub prefix: &'a str,
    pub suffix: &'a str,
}

/// FromStr accepting a string with more data.
///
/// It does not assume the string only contain the note data, but
/// tolerate and retain a prefix and a suffix.
pub trait PartialFromStr
where
    Self: Sized,
{
    type Err;

    /// Try parsing a value from a string.
    fn partial_from_str<'a>(s: &'a str) -> Result<Parsed<'a, Self>, Self::Err>;
}

impl PartialFromStr for MidiNote {
    type Err = anyhow::Error;

    fn partial_from_str<'a>(s: &'a str) -> Result<Parsed<'a, Self>, Self::Err> {
        let note = parse_letter_notation(s).or_else(|| parse_number_notation(s));
        if let Some(note) = note {
            return Ok(Parsed {
                value: note,
                prefix: "",
                suffix: "",
            });
        }
        anyhow::bail!("Not a note")
    }
}

fn parse_number_notation(filename: &str) -> Option<MidiNote> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"1[0-2]\d|\d\d|\d").expect("BUG: Invalid number notation regex");
    }

    let capture = RE.captures(filename)?;
    let number = capture[0].parse::<u8>().ok()?;
    Some(MidiNote::from(number))
}

fn parse_letter_notation(filename: &str) -> Option<MidiNote> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<prefix>.*)(?:(?i)[^A-Z]|^)(?P<letter>(?i)[A-G])(?P<accidental>#?b?)(?P<octave>10|-?[0-9])(?P<suffix>.*)"
        )
        .expect("BUG: Invalid letter notation regex");
    }

    let capture = RE.captures(filename)?;
    dbg!(&capture);

    let letter = capture
        .name("letter")
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

    let natural = match letter {
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
        assert_eq!(parse_letter_notation(input), Some(expected));
    }

    #[rstest]
    #[case("THMB40.wav", MidiNote::from(40))]
    #[case("THMB43.wav", MidiNote::from(43))]
    #[case("THMB48.wav", MidiNote::from(48))]
    fn parse_number_notation_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(parse_number_notation(input), Some(expected));
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
    fn parse_note_test(#[case] input: &str, #[case] expected: MidiNote) {
        assert_eq!(
            MidiNote::partial_from_str(input)
                .expect("Failed to parse valid note")
                .value,
            expected
        );
    }

    #[test]
    fn parse_not_a_note() {
        MidiNote::partial_from_str("nope.wav").unwrap_err();
    }
}
