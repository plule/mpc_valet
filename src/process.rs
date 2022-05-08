use music_note::{midi::MidiNote, Interval};

use crate::parse::NoteFile;

#[derive(PartialEq, Debug)]
pub struct Keygroup {
    pub note: NoteFile,
    pub range: Range,
}

impl Keygroup {
    pub fn new(note: NoteFile, range: Range) -> Self {
        Self { note, range }
    }
}

#[derive(PartialEq, Debug)]
pub struct Range {
    pub low: MidiNote,
    pub high: MidiNote,
}

impl Range {
    pub fn new(low: MidiNote, high: MidiNote) -> Self {
        Self { low, high }
    }
}

pub fn build_ranges<'a, I>(mut notes: I) -> Vec<Range>
where
    I: IntoIterator<Item = &'a MidiNote>,
{
    let mut notes: Vec<&MidiNote> = notes.into_iter().collect();
    notes.sort();

    // Create all the interval cuts. It always have at least the min and max midi notes,
    // then all the half way cuts between roots.
    let mut cuts = vec![MidiNote::from(u8::MIN)];
    cuts.extend(notes.windows(2).map(|w| {
        let root1 = *w[0];
        let root2 = *w[1];
        let half_distance = Interval::new((root2 - root1).semitones() / 2);
        root1 + half_distance
    }));
    cuts.push(MidiNote::from(u8::MAX));

    // Build the corresponding ranges with no overlap
    cuts.windows(2)
        .map(|w| {
            let low = w[0] + Interval::new(1);
            let high = w[1];

            Range::new(low, high)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_program() {
        let notes = vec![MidiNote::from(45), MidiNote::from(57), MidiNote::from(69)];

        let ranges = build_ranges(&notes);
        let expected_ranges = vec![
            Range::new(MidiNote::from(1), MidiNote::from(51)),
            Range::new(MidiNote::from(52), MidiNote::from(63)),
            Range::new(MidiNote::from(64), MidiNote::from(255)),
        ];

        assert_eq!(ranges, expected_ranges);
    }
}
