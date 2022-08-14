use std::ops::RangeInclusive;

use staff::{midi::MidiNote, Interval};

/// Create an appropriate set of ranges from midi notes
pub fn build_ranges<'a, I>(notes: I, pitch_preference: f32) -> Vec<RangeInclusive<u8>>
where
    I: IntoIterator<Item = &'a u8>,
{
    let notes: Vec<&u8> = notes.into_iter().collect();

    if notes.is_empty() {
        return Vec::<RangeInclusive<u8>>::new();
    }

    // Create all the interval cuts. It always have at least the min and max midi notes,
    // then all the half way cuts between roots.
    let mut cuts = vec![MidiNote::from(0)];
    cuts.extend(notes.windows(2).map(|w| {
        let root1 = MidiNote::from_byte(*w[0]);
        let root2 = MidiNote::from_byte(*w[1]);
        assert!(root2 >= root1);
        if root1 == root2 {
            return root1;
        }
        let distance = (root2 - root1).semitones();
        let cut_point = Interval::new(
            ((pitch_preference * distance as f32) as u8)
                .max(0)
                .min(distance - 1),
        );
        root1 + cut_point
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
            low.into_byte()..=high.into_byte()
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
        vec![0..=51, 52..=63, 64..=127],
        0.5,
    )]
    #[case(
        vec![45, 57, 69],
        vec![0..=45, 46..=57, 58..=127],
        0.0,
    )]
    #[case(
        vec![45, 57, 69],
        vec![0..=56, 57..=68, 69..=127],
        1.0,
    )]
    #[case(
        vec![45, 46, 47, 48],
        vec![0..=45, 46..=46, 47..=47, 48..=127],
        0.5,
    )]
    #[case(
        vec![45],
        vec![0..=127],
        0.5,
    )]
    #[case(
        vec![],
        vec![],
        0.5,
    )]
    fn test_build_ranges(
        #[case] input: Vec<u8>,
        #[case] expected: Vec<RangeInclusive<u8>>,
        #[case] pitch_preference: f32,
    ) {
        let ranges = build_ranges(&input, pitch_preference);
        assert_eq!(ranges, expected);
    }
}
