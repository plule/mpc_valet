use crate::components::*;
use crate::model::{KeygroupProgram, LayerFile, SampleFile};
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use log::debug;
use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;

pub enum Msg {
    Reset,
    AddSamplesDone(Vec<SampleFile>),
    SelectLayersDone(Vec<LayerFile>),
    FineTuningDone(KeygroupProgram),
}

/// Wizard steps
#[derive(Serialize, Deserialize)]
pub enum Step {
    /// Selecting samples and their root notes
    AddSamples,

    /// The file roots were selected
    SelectLayers(Vec<SampleFile>),

    /// Fine Tuning of the program
    FineTuning(Vec<LayerFile>),

    /// The program is ready to be saved
    Done(KeygroupProgram),
}

/// Main component: Create the keygroup programs.
#[derive(Serialize, Deserialize)]
pub struct KeygroupCreator {
    /// Steps of the keygroup creation
    step: Step,
}

impl Default for KeygroupCreator {
    fn default() -> Self {
        Self {
            step: Step::AddSamples,
        }
    }
}

impl Component for KeygroupCreator {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        LocalStorage::get("keygroup_creator").unwrap_or_default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            Msg::Reset => {
                *self = Self::default();
                LocalStorage::clear();
                true
            }
            Msg::AddSamplesDone(samples) => {
                self.step = Step::SelectLayers(samples);
                true
            }
            Msg::SelectLayersDone(layer_files) => {
                self.step = Step::FineTuning(layer_files);
                true
            }
            Msg::FineTuningDone(program) => {
                self.step = Step::Done(program);
                true
            }
        };

        LocalStorage::set("keygroup_creator", self).unwrap_or_else(|e| {
            log::error!("{e}");
        });

        redraw
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut add_samples_class = classes!("steps-segment");
        let mut select_layers_class = classes!("steps-segment");
        let mut fine_tuning_class = classes!("steps-segment");
        let mut done_class = classes!("steps-segment");

        match self.step {
            Step::AddSamples => add_samples_class.push("is-active"),
            Step::SelectLayers(_) => select_layers_class.push("is-active"),
            Step::FineTuning(_) => fine_tuning_class.push("is-active"),
            Step::Done(_) => done_class.push("is-active"),
        }

        debug!("Redrawing main view");
        html! {
        <div class="container">
            <div class="box">
                <ul class="steps has-content-centered">
                    <li class={add_samples_class}>
                        <span class="steps-marker">
                            <Icon icon="musical-note" />
                        </span>
                        <div class="steps-content">
                            <p class="is-size-4">{"Add Samples"}</p>
                        </div>
                    </li>
                    <li class={select_layers_class}>
                        <span class="steps-marker">
                            <Icon icon="layers" />
                        </span>
                        <div class="steps-content">
                            <p class="is-size-4">{"Select Layers"}</p>
                        </div>
                    </li>
                    <li class={fine_tuning_class}>
                        <span class="steps-marker">
                            <Icon icon="options" />
                        </span>
                        <div class="steps-content">
                            <p class="is-size-4">{"Fine Tuning"}</p>
                        </div>
                    </li>
                    <li class={done_class}>
                        <span class="steps-marker">
                            <Icon icon="checkmark" />
                        </span>
                        <div class="steps-content">
                            <p class="is-size-4">{"Done!"}</p>
                        </div>
                    </li>
                </ul>
                {self.view_current_step(ctx)}
            </div>
            <div class="buttons is-centered">
                <button class="button is-danger is-large" onclick={ctx.link().callback(|_| Msg::Reset)}>
                    <Icon icon="trash" text_after="Reset" />
                </button>
            </div>
        </div>
        }
    }
}

impl KeygroupCreator {
    fn view_current_step(&self, ctx: &Context<Self>) -> Html {
        match &self.step {
            Step::AddSamples => html! {
                <StepAddSamples
                    on_next={ctx.link().callback(Msg::AddSamplesDone)}
                />
            },
            Step::SelectLayers(files) => {
                html! {
                    <StepSelectLayers
                        files={files.clone()}
                        on_next={ctx.link().callback(Msg::SelectLayersDone)}
                    />
                }
            }
            Step::FineTuning(layer_files) => {
                html! {
                    <StepFineTuning
                        layer_files = {layer_files.clone()}
                        on_next = {ctx.link().callback(Msg::FineTuningDone)}
                    />
                }
            }
            Step::Done(program) => html! {
                <StepDone
                    program = {program.clone()}
                />
            },
        }
    }
}
