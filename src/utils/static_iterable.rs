///! Static Iterable trait
use std::slice::Iter;

use lazy_static::lazy_static;
use staff::{
    midi::{MidiNote, Octave},
    Pitch,
};

/// Ability to statically iterate over all the possible values.
pub trait StaticIterable
where
    Self: Sized,
{
    /// Iterate over all the possible values.
    fn iter() -> Iter<'static, Self>;
}

impl StaticIterable for Octave {
    fn iter() -> Iter<'static, Self> {
        static OCTAVES: [Octave; 10] = [
            Octave::NEGATIVE_ONE,
            Octave::ZERO,
            Octave::ONE,
            Octave::TWO,
            Octave::THREE,
            Octave::FOUR,
            Octave::FIVE,
            Octave::SIX,
            Octave::SEVEN,
            Octave::EIGHT,
        ];
        OCTAVES.iter()
    }
}

impl StaticIterable for Pitch {
    fn iter() -> Iter<'static, Self> {
        static PITCHES: [Pitch; 12] = [
            Pitch::C,
            Pitch::CSharp,
            Pitch::D,
            Pitch::DSharp,
            Pitch::E,
            Pitch::F,
            Pitch::FSharp,
            Pitch::G,
            Pitch::GSharp,
            Pitch::A,
            Pitch::ASharp,
            Pitch::B,
        ];
        PITCHES.iter()
    }
}

impl StaticIterable for MidiNote {
    fn iter() -> Iter<'static, Self> {
        lazy_static! {
            static ref MIDI_NOTES: Vec<MidiNote> = (0..=127).map(MidiNote::from).collect();
        }

        MIDI_NOTES.iter()
    }
}
