use std::fmt::Display;

use music_note::midi::MidiNote;
use yew::{html, Callback, Component, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct LayerSelectProps {
    pub label: String,
    pub initial: Option<usize>,
    pub selection_changed: Callback<usize>,
}

pub enum LayerSelectMessage {
    SelectionChanged(usize),
}

/// Single layer selector.
pub struct LayerSelect;

impl Component for LayerSelect {
    type Message = LayerSelectMessage;

    type Properties = LayerSelectProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let LayerSelectProps {
            label,
            initial,
            selection_changed: _,
        } = ctx.props().clone();

        let buttons: Html = (0..4).map(|layer| {
            let class = if Some(layer) == initial {
                "button is-primary"
            } else {
                "button"
            };
            html! {
                <button
                    class={class}
                    onclick={ctx.link().callback(move |_| LayerSelectMessage::SelectionChanged(layer))}
                >{format!("Layer {}", layer+1)}</button>
            }
        }).collect();

        html! {
            <div class="tile">
                <div class="tile is-4">
                    {label}
                </div>
                <div class="tile">
                    <div class="buttons has-addons">
                        {buttons}
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LayerSelectMessage::SelectionChanged(layer) => {
                ctx.props().selection_changed.emit(layer);
                false
            }
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
