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

/// Possible stages when adding files
#[derive(Serialize, Deserialize)]
pub enum Stage {
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
    /// Stage of the keygroup creation
    stage: Stage,
}

impl Default for KeygroupCreator {
    fn default() -> Self {
        Self {
            stage: Stage::AddSamples,
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
                self.stage = Stage::SelectLayers(samples);
                true
            }
            Msg::SelectLayersDone(layer_files) => {
                self.stage = Stage::FineTuning(layer_files);
                true
            }
            Msg::FineTuningDone(program) => {
                self.stage = Stage::Done(program);
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

        match self.stage {
            Stage::AddSamples => add_samples_class.push("is-active"),
            Stage::SelectLayers(_) => select_layers_class.push("is-active"),
            Stage::FineTuning(_) => fine_tuning_class.push("is-active"),
            Stage::Done(_) => done_class.push("is-active"),
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
                {self.view_current_stage(ctx)}
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
    fn view_current_stage(&self, ctx: &Context<Self>) -> Html {
        match &self.stage {
            Stage::AddSamples => html! {
                <StepAddSamples
                    on_next={ctx.link().callback(Msg::AddSamplesDone)}
                />
            },
            Stage::SelectLayers(files) => {
                html! {
                    <StepSelectLayers
                        files={files.clone()}
                        on_next={ctx.link().callback(|layer_files| Msg::SelectLayersDone(layer_files))}
                    />
                }
            }
            Stage::FineTuning(layer_files) => {
                html! {
                    <StepFineTuning
                        layer_files = {layer_files.clone()}
                        on_next = {ctx.link().callback(|program| Msg::FineTuningDone(program))}
                    />
                }
            }
            Stage::Done(program) => html! {
                <StepDone
                    program = {program.clone()}
                />
            },
        }
    }
}
