use crate::components::{Icon, NoteSelect};
use gloo_storage::{LocalStorage, Storage};
use music_note::midi::MidiNote;
use serde::{Deserialize, Serialize};
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::SampleFile;

#[derive(Properties, PartialEq)]
pub struct RootNoteFormProps {
    #[prop_or_default]
    pub files: Vec<String>,

    #[prop_or_default]
    pub on_done: Callback<Vec<SampleFile>>,

    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

pub enum RootNoteFormMessages {
    RootNoteChanged(usize, MidiNote),
    IncreaseOctave,
    DecreaseOctave,
    Done,
    Reset,
    Cancel,
}

/// Root note selector for a list of sample files.
#[derive(Default, Serialize, Deserialize)]
pub struct RootNotesForm {
    pub sample_files: Vec<SampleFile>,
}

impl From<Vec<String>> for RootNotesForm {
    fn from(files: Vec<String>) -> Self {
        Self {
            sample_files: files.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl Component for RootNotesForm {
    type Message = RootNoteFormMessages;
    type Properties = RootNoteFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        LocalStorage::get("root_note_form").unwrap_or_else(|_| ctx.props().files.clone().into())
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("root_note_form");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            RootNoteFormMessages::RootNoteChanged(index, note) => {
                self.sample_files[index].root = note.into_byte();
                true
            }
            RootNoteFormMessages::IncreaseOctave => {
                self.sample_files.iter_mut().for_each(|s| {
                    s.root = (s.root + 12).clamp(0, 127);
                });
                true
            }
            RootNoteFormMessages::DecreaseOctave => {
                self.sample_files.iter_mut().for_each(|s| {
                    s.root = (s.root - 12).clamp(0, 127);
                });
                true
            }
            RootNoteFormMessages::Done => {
                ctx.props().on_done.emit(self.sample_files.clone());
                false
            }
            RootNoteFormMessages::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
            RootNoteFormMessages::Reset => {
                *self = ctx.props().files.clone().into();
                true
            }
        };

        LocalStorage::set("root_note_form", self).unwrap_or_else(|e| {
            log::error!("{e}");
        });

        redraw
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let samples: Vec<Html> = self
            .sample_files
            .iter()
            .enumerate()
            .map(|(index, sample)| {
                html! {
                    <div class="tile">
                        <div class="tile is-4">
                            {&sample.file}
                        </div>
                        <div class="tile">
                            <div class="control select">
                                <NoteSelect
                                    value={MidiNote::from_byte(sample.root)}
                                    selection_changed={ctx.link().callback(move |root: MidiNote| RootNoteFormMessages::RootNoteChanged(index, root))}
                                />
                            </div>
                        </div>
                    </div>
                }
            })
            .collect();

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <Icon icon="musical-notes" text="Select root notes" text_class="modal-card-title" />
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| RootNoteFormMessages::Cancel)}></button>
                    </header>
                    <section class="modal-card-body">
                        <div class="tile is-ancestor is-vertical">
                            {samples}
                        </div>
                    </section>
                    <footer class="modal-card-foot is-centered">
                        <div class="columns">
                            <section class="section">
                                <div class="buttons has-addons">
                                    <button class="button" onclick={ctx.link().callback(|_| RootNoteFormMessages::DecreaseOctave)}>
                                        <Icon icon="remove-circle-outline" text="Octave -" />
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| RootNoteFormMessages::IncreaseOctave)}>
                                        <Icon icon="add-circle-outline" text="Octave +" />
                                    </button>
                                </div>
                            </section>
                            <section class="section">
                                <div class="buttons has-addons">
                                    <button class="button is-danger" onclick={ctx.link().callback(|_| RootNoteFormMessages::Cancel)}>
                                        <Icon icon="trash" text="Cancel" />
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| RootNoteFormMessages::Reset)}>
                                        <Icon icon="refresh" text="Reset" />
                                    </button>
                                    <button class="button is-success" onclick={ctx.link().callback(|_| RootNoteFormMessages::Done)}>{"Next"}</button>
                                </div>
                            </section>
                        </div>
                    </footer>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        *self = ctx.props().files.clone().into();
        true
    }
}
