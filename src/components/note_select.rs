use std::fmt::Display;

use music_note::midi::MidiNote;
use yew::{html, Callback, Component, Properties};
use yew_utils::components::drop_down::DropDown;

use crate::utils::StaticIterable;

#[derive(Properties, PartialEq, Clone)]
pub struct NoteSelectProps {
    pub initial: MidiNote,
    pub selection_changed: Callback<MidiNote>,
}

/// Single midi note selector drop down.
pub struct NoteSelect;

impl Component for NoteSelect {
    type Message = ();

    type Properties = NoteSelectProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let NoteSelectProps {
            initial,
            selection_changed,
        } = ctx.props().clone();

        let root_options: Vec<MidiNoteWithDisplay> =
            MidiNote::iter().map(|note| (*note).into()).collect();

        let on_selection_changed =
            Callback::from(move |note: MidiNoteWithDisplay| selection_changed.emit(note.0));

        html! {
            <div class="select">
                <DropDown<MidiNoteWithDisplay>
                    initial={MidiNoteWithDisplay(initial)}
                    options={root_options.clone()}
                    selection_changed={on_selection_changed}
                />
            </div>
        }
    }
}

#[derive(PartialEq, Clone)]
struct MidiNoteWithDisplay(MidiNote);

impl Display for MidiNoteWithDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0.pitch(), self.0.octave())
    }
}

impl From<MidiNote> for MidiNoteWithDisplay {
    fn from(note: MidiNote) -> Self {
        Self(note)
    }
}
