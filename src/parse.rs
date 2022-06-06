use lazy_static::lazy_static;
use music_note::{
    midi::{MidiNote, Octave},
    note::{Accidental, AccidentalKind, Flat, Sharp},
    Natural, Pitch,
};
use regex::Regex;

pub fn parse_number_notation(filename: &str) -> Option<MidiNote> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"1[0-2]\d|\d\d|\d").expect("BUG: Invalid number notation regex");
    }

    let capture = RE.captures(filename)?;
    let number = capture[0].parse::<u8>().ok()?;
    Some(MidiNote::from(number))
}

pub fn parse_letter_notation(filename: &str) -> Option<MidiNote> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?:[^A-Z]|^)(?P<letter>[A-G])(?P<accidental>#?b?)(?P<octave>10|-?[0-9])")
                .expect("BUG: Invalid letter notation regex");
    }

    let capture = RE.captures(filename)?;

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
        "A" => Some(Natural::A),
        "B" => Some(Natural::B),
        "C" => Some(Natural::C),
        "D" => Some(Natural::D),
        "E" => Some(Natural::E),
        "F" => Some(Natural::F),
        "G" => Some(Natural::G),
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

pub fn parse_note(filename: &str) -> Option<MidiNote> {
    parse_letter_notation(filename).or_else(|| parse_number_notation(filename))
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
    #[case("MELCEL-F4.WAV", midi!(F,4))]
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
        assert_eq!(parse_note(input), Some(expected));
    }

    #[test]
    fn parse_not_a_note() {
        assert_eq!(parse_note("nope.wav"), None);
    }
}
