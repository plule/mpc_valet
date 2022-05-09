use music_note::{midi::MidiNote, Interval};

use crate::Range;

/// Create an appropriate set of ranges from midi notes
pub fn build_ranges<'a, I>(notes: I) -> Vec<Range>
where
    I: IntoIterator<Item = &'a MidiNote>,
{
    let notes: Vec<&MidiNote> = notes.into_iter().collect();

    if notes.is_empty() {
        return Vec::<Range>::new();
    }

    // Create all the interval cuts. It always have at least the min and max midi notes,
    // then all the half way cuts between roots.
    let mut cuts = vec![MidiNote::from(0)];
    cuts.extend(notes.windows(2).map(|w| {
        let root1 = *w[0];
        let root2 = *w[1];
        assert!(root2 >= root1);
        let half_distance = Interval::new((root2 - root1).semitones() / 2);
        root1 + half_distance
    }));
    cuts.push(MidiNote::from(127));

    // Build the corresponding ranges with no overlap
    cuts.windows(2)
        .map(|w| {
            let low = w[0]
                + if w[0].into_byte() == 0 {
                    Interval::new(0) // No overlap for the first midi note
                } else {
                    Interval::new(1)
                };
            let high = w[1];

            Range::new(low, high)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        vec![45, 57, 69],
        vec![(0,51), (52,63), (64,127)],
    )]
    #[case(
        vec![45, 46, 47, 48],
        vec![(0,45), (46,46), (47,47), (48,127)],
    )]
    #[case(
        vec![45],
        vec![(0,127)],
    )]
    #[case(
        vec![],
        vec![],
    )]
    fn test_build_ranges(#[case] input: Vec<u8>, #[case] expected: Vec<(u8, u8)>) {
        let input: Vec<MidiNote> = input.iter().map(|semis| MidiNote::from(*semis)).collect();
        let expected: Vec<Range> = expected
            .iter()
            .map(|(low, high)| Range::new(MidiNote::from(*low), MidiNote::from(*high)))
            .collect();
        let ranges = build_ranges(&input);
        assert_eq!(ranges, expected);
    }
}
