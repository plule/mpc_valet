use std::fmt::Display;

use staff::midi::MidiNote;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};
use yew::{html, Callback, Component, Html, NodeRef, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct NoteSelectProps {
    pub value: MidiNote,
    pub selection_changed: Callback<MidiNote>,
}

/// Single midi note selector drop down.
pub struct NoteSelect {
    node_ref: NodeRef,
}

impl Component for NoteSelect {
    type Message = ();

    type Properties = NoteSelectProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let NoteSelectProps {
            value,
            selection_changed,
        } = ctx.props().clone();

        let on_change = move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<HtmlSelectElement>() {
                    let byte = select.selected_index() as u8;
                    let note = MidiNote::from_byte(byte);
                    selection_changed.emit(note);
                }
            }
        };

        let options: Html = (0..=127_u8).into_iter()
            .map(|byte| {
                let note = MidiNote::from_byte(byte);
                let opt_str = format!("{}{}", note.pitch(), note.octave());
                html! {
                    <>
                    <p class="navbar-item">
                        <div class="dropdown is-active">
                            <div class="dropdown-trigger mx-2">
                                {"Submenu 1&nbsp;&nbsp;&nbsp;"}
                                <i class="fas fa-angle-right"></i>
                            </div>
                        <div class="dropdown-menu" role="menu" style="top:-15px;margin-left:100%;">
                            <div class="dropdown-content">
                                <a href="#" class="dropdown-item">{"coso 2"}</a>
                                <a href="#" class="dropdown-item">{"coso"}</a>
                                <a href="#" class="dropdown-item">{"cosito"}</a>
                            </div>
                        </div>
                        </div>
                    </p>
                    <option value={opt_str.clone()} selected={note == value}>{opt_str.clone()}</option>
                    </>
                }
            })
            .collect();

        html! {
            <div class="select" onchange={on_change}>
                <select ref={self.node_ref.clone()}>
                    {options}
                </select>
            </div>
        }
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        if let Some(elt) = self.node_ref.cast::<HtmlSelectElement>() {
            let value = ctx.props().value;
            elt.set_value(&format!("{}{}", value.pitch(), value.octave()));
        }
        true
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
