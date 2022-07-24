use crate::{
    components::{Icon, Keyboard, KeygroupsTable},
    model::{KeygroupProgram, LayerFile, LayerVelocityMode},
};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_utils::components::drop_down::DropDown;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub layer_files: Vec<LayerFile>,

    #[prop_or_default]
    pub on_previous: Callback<()>,

    #[prop_or_default]
    pub on_next: Callback<KeygroupProgram>,
}

pub enum Msg {
    PitchPreferenceChange(f32),
    LayerVelocityModeChange(LayerVelocityMode),
    HighlightKeygroup(Option<usize>),
    Previous,
    Next,
}

/// Root note selector for a list of sample files.
#[derive(Default, Serialize, Deserialize)]
pub struct StepFineTuning {
    /// Pitch preference (0 to 1, prefer pitching down or up)
    pitch_preference: f32,

    /// Layer velocity mode
    layer_velocity_mode: LayerVelocityMode,

    /// The keygroup program being tuned
    program: KeygroupProgram,

    /// Keygroup index to highlight
    highlight_keygroup: Option<usize>,
}

impl From<Vec<LayerFile>> for StepFineTuning {
    fn from(layer_files: Vec<LayerFile>) -> Self {
        let mut program = KeygroupProgram::default();
        program.insert_layer_files(layer_files);
        program.sort_keygroups();
        program.guess_ranges(0.5);

        Self {
            program,
            pitch_preference: 0.5,
            layer_velocity_mode: LayerVelocityMode::Automatic,
            highlight_keygroup: None,
        }
    }
}

impl Component for StepFineTuning {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        LocalStorage::get("step_fine_tuning")
            .unwrap_or_else(|_| ctx.props().layer_files.clone().into())
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("step_fine_tuning");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            Msg::PitchPreferenceChange(pitch_preference) => {
                self.pitch_preference = pitch_preference;
                self.program.guess_ranges(pitch_preference);
                true
            }
            Msg::LayerVelocityModeChange(mode) => {
                self.program.set_velocity_layer_mode(&mode);
                self.layer_velocity_mode = mode;
                true
            }
            Msg::HighlightKeygroup(index) => {
                self.highlight_keygroup = index;
                true
            }
            Msg::Previous => {
                ctx.props().on_previous.emit(());
                false
            }
            Msg::Next => {
                ctx.props().on_next.emit(self.program.clone());
                false
            }
        };

        LocalStorage::set("step_fine_tuning", self).unwrap_or_else(|e| {
            log::error!("{e}");
        });

        redraw
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let layer_help_text = match self.layer_velocity_mode {
            LayerVelocityMode::Automatic => {
                "Each layer will only be used for a range of the velocity."
            }
            LayerVelocityMode::Unison => "All the layers will play at the same time.",
        };
        html! {
            <>
                <Keyboard keygroups={self.program.keygroups.clone()} highlight_keygroup={self.highlight_keygroup} />
                <KeygroupsTable keygroups={self.program.keygroups.clone()} on_hovered_kg={ctx.link().callback(Msg::HighlightKeygroup)} />
                <div class="block">
                    <div class="field">
                        <label class="label">{"Pitch Preference"}</label>
                        <div class="control">
                            <input
                                id="pitch_preference"
                                type="range"
                                min=0
                                max=1
                                step=0.01
                                value={self.pitch_preference.to_string()}
                                oninput={StepFineTuning::on_pitch_preference_change(ctx)}
                            />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{"Layer Velocity Mode"}</label>
                        <div class="control">
                            <div class="select">
                                <DropDown<LayerVelocityMode>
                                    initial={self.layer_velocity_mode}
                                    options={vec![LayerVelocityMode::Unison, LayerVelocityMode::Automatic]}
                                    selection_changed={ctx.link().callback(Msg::LayerVelocityModeChange)}
                                />
                            </div>
                        </div>
                        <p class="help">{layer_help_text}</p>
                    </div>
                </div>
                <div class="buttons has-addons is-centered">
                    /*<button class="button" onclick={ctx.link().callback(|_| Msg::Previous)}>
                        <Icon icon="caret-back" text_after ="Previous" />
                    </button>*/
                    <button class="button is-success" onclick={ctx.link().callback(|_| Msg::Next)}>
                        <Icon icon="caret-forward" text_before="Next" />
                    </button>
                </div>
            </>
        }
    }
}

impl StepFineTuning {
    fn on_pitch_preference_change(ctx: &Context<StepFineTuning>) -> Callback<InputEvent> {
        ctx.link().batch_callback(|e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            Some(Msg::PitchPreferenceChange(input.value_as_number() as f32))
        })
    }
}
