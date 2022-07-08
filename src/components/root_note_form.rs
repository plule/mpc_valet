use crate::components::NoteSelect;
use log::debug;
use music_note::midi::MidiNote;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::SampleFile;

#[derive(Properties, PartialEq)]
pub struct RootNoteFormProps {
    #[prop_or_default]
    pub files: Vec<String>,

    #[prop_or_default]
    pub on_selected: Callback<Vec<SampleFile>>,

    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

pub enum RootNoteFormMessages {
    RootNoteChanged(usize, MidiNote),
    Ok,
    Cancel,
}

/// Root note selector for a list of sample files.
#[derive(Default)]
pub struct RootNotesForm {
    pub sample_files: Vec<SampleFile>,
}

impl Component for RootNotesForm {
    type Message = RootNoteFormMessages;
    type Properties = RootNoteFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        debug!("recreating root note form");
        Self {
            sample_files: ctx.props().files.iter().map(|f| f.clone().into()).collect(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootNoteFormMessages::RootNoteChanged(index, note) => {
                self.sample_files[index].root = note;
                true
            }
            RootNoteFormMessages::Ok => {
                ctx.props().on_selected.emit(self.sample_files.clone());
                false
            }
            RootNoteFormMessages::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
        }
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
                                    initial={sample.root}
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
                        <span class="icon">
                            <ion-icon name="musical-notes"></ion-icon>
                        </span>
                        <p class="modal-card-title">{"Select root notes"}</p>
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| RootNoteFormMessages::Cancel)}></button>
                    </header>
                    <section class="modal-card-body">
                        <div class="tile is-ancestor is-vertical">
                            {samples}
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-success" onclick={ctx.link().callback(|_| RootNoteFormMessages::Ok)}>{"Ok"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| RootNoteFormMessages::Cancel)}>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.sample_files = ctx.props().files.iter().map(|f| f.clone().into()).collect();
        true
    }
}
