use crate::components::{Icon, NoteSelect};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use staff::midi::MidiNote;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::SampleFile;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_next: Callback<Vec<SampleFile>>,
}

pub enum Msg {
    FilesDropped(Vec<String>),
    RootNoteChanged(usize, MidiNote),
    IncreaseOctave,
    DecreaseOctave,
    GuessRoots,
    Next,
}

/// Root note selector for a list of sample files.
#[derive(Default, Serialize, Deserialize)]
pub struct StepAddSamples {
    pub sample_files: Vec<SampleFile>,
}

impl From<Vec<String>> for StepAddSamples {
    fn from(files: Vec<String>) -> Self {
        Self {
            sample_files: files.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl Component for StepAddSamples {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        LocalStorage::get("add_samples_form").unwrap_or_default()
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("add_samples_form");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            Msg::RootNoteChanged(index, note) => {
                self.sample_files[index].root = note.into_byte();
                true
            }
            Msg::IncreaseOctave => {
                self.sample_files.iter_mut().for_each(|s| {
                    s.root = (s.root + 12).clamp(0, 127);
                });
                true
            }
            Msg::DecreaseOctave => {
                self.sample_files.iter_mut().for_each(|s| {
                    s.root = (s.root - 12).clamp(0, 127);
                });
                true
            }
            Msg::Next => {
                ctx.props().on_next.emit(self.sample_files.clone());
                false
            }
            Msg::GuessRoots => {
                self.sample_files.iter_mut().for_each(|f| f.guess_root());
                true
            }
            Msg::FilesDropped(files) => {
                files
                    .into_iter()
                    .for_each(|f| self.sample_files.push(f.into()));
                true
            }
        };

        LocalStorage::set("add_samples_form", self).unwrap_or_else(|e| {
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
                        <div class="tile">
                            {&sample.file}
                        </div>
                        <div class="tile">
                            <div class="control select">
                                <NoteSelect
                                    value={MidiNote::from_byte(sample.root)}
                                    selection_changed={ctx.link().callback(move |root: MidiNote| Msg::RootNoteChanged(index, root))}
                                />
                            </div>
                        </div>
                    </div>
                }
            })
            .collect();

        html! {
            <>
            <section class="section">
                <div class="buttons has-addons is-centered">
                    <button class="button" onclick={ctx.link().callback(|_| Msg::GuessRoots)}>
                        <Icon icon="color-wand" text_after="Guess Roots" />
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::DecreaseOctave)}>
                        <Icon icon="remove-circle-outline" text_after="Octave -" />
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::IncreaseOctave)}>
                        <Icon icon="add-circle-outline" text_after="Octave +" />
                    </button>
                </div>
            </section>
            <div class="columns">
                <div class="column"></div>
                <div class="column is-half">
                    <div class="tile is-ancestor is-vertical">
                        {samples}
                    </div>
                </div>
                <div class="column"></div>
            </div>
            <div class="file is-boxed is-centered is-primary">
                <label class="file-label">
                    <input class="file-input" type="file" name="samples" multiple=true accept=".wav" oninput={StepAddSamples::on_file_input(ctx)} />
                    <span class="file-cta">
                        <Icon icon="add-circle" class="file-icon" text_after="Add Samples..." text_class="file-label" />
                    </span>
                </label>
            </div>
            <div class="columns is-centered">
                <section class="section">
                    <div class="buttons has-addons">
                        <button class="button is-success" onclick={ctx.link().callback(|_| Msg::Next)}>{"Next"}</button>
                    </div>
                </section>
            </div>
            </>
        }
    }
}

impl StepAddSamples {
    fn on_file_input(ctx: &Context<StepAddSamples>) -> Callback<InputEvent> {
        ctx.link().batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            let files = input.files()?;
            let file_names: Vec<String> = (0..files.length())
                .filter_map(|i| Some(files.get(i)?.name()))
                .collect();
            Some(Msg::FilesDropped(file_names))
        })
    }
}
