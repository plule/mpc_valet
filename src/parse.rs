use itertools::Itertools;
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
            Regex::new(r"(?P<letter>[A-G])(?P<accidental>#?b?)(?P<octave>10|-?[0-9])")
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

/// Parse a list of file names and return their midi notes.
pub fn find_best_candidate<'a, I>(filenames: I) -> Vec<Option<MidiNote>>
where
    I: IntoIterator<Item = &'a str>,
{
    let (letter_notation_result, midi_number_result): (
        Vec<Option<MidiNote>>,
        Vec<Option<MidiNote>>,
    ) = filenames
        .into_iter()
        .map(|f| (parse_letter_notation(f), parse_number_notation(f)))
        .unzip();

    if letter_notation_result.iter().all(|r| r.is_some())
        && letter_notation_result
            .iter()
            .map(|r| r.unwrap().into_byte())
            .all_unique()
    {
        // The letter notation returned unique results for all the samples, good enough
        return letter_notation_result;
    }

    if midi_number_result.iter().all(|r| r.is_some())
        && midi_number_result
            .iter()
            .map(|r| r.unwrap().into_byte())
            .all_unique()
    {
        // Otherwise, if all the midi number notation are OK, find by me
        return midi_number_result;
    }

    // No "clean" match, just return what we can, privileging letter notation just in case
    letter_notation_result
        .into_iter()
        .zip(midi_number_result.into_iter())
        .map(|(l, m)| l.or(m))
        .collect()
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
    #[case(
        vec!["THMB40.WAV", "THMB43.WAV", "THMB48.WAV"],
        vec![Some(MidiNote::from(40)), Some(MidiNote::from(43)), Some(MidiNote::from(48))],
    )]
    #[case(
        vec!["THMB-40.WAV", "THMB-43.WAV", "MELCEL-A2.WAV", "MELCEL-A3.WAV", "nope.WAV"],
        vec![Some(MidiNote::from(40)), Some(MidiNote::from(43)), Some(midi!(A, 2)), Some(midi!(A, 3)), None],
    )]
    fn parse_roots(#[case] input: Vec<&str>, #[case] expected: Vec<Option<MidiNote>>) {
        let midi_notes = find_best_candidate(input);
        assert_eq!(midi_notes, expected);
    }

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
}
