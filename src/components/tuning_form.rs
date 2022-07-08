use crate::model::LayerVelocityMode;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_utils::components::drop_down::DropDown;

#[derive(Properties, PartialEq)]
pub struct TuningFormProps {
    #[prop_or_default]
    pub pitch_preference_init: f32,

    #[prop_or_default]
    pub layer_velocity_mode_init: LayerVelocityMode,

    #[prop_or_default]
    pub program_name_init: String,

    #[prop_or_default]
    pub on_pitch_preference_change: Callback<f32>,

    #[prop_or_default]
    pub on_layer_velocity_mode_change: Callback<LayerVelocityMode>,

    #[prop_or_default]
    pub on_program_name_change: Callback<String>,

    #[prop_or_default]
    pub on_save: Callback<Tuning>,
}

pub enum TuningFormMessages {
    PitchPreferenceChange(f32),
    LayerVelocityModeChange(LayerVelocityMode),
    ProgramNameChanged(String),
    Save,
}

/// Root note selector for a list of sample files.
#[derive(Default)]
pub struct TuningForm {
    pub tuning: Tuning,
}

#[derive(Default, Clone)]
pub struct Tuning {
    pub pitch_preference: f32,
    pub layer_velocity_mode: LayerVelocityMode,
    pub program_name: String,
}

impl Component for TuningForm {
    type Message = TuningFormMessages;
    type Properties = TuningFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tuning: Tuning {
                pitch_preference: ctx.props().pitch_preference_init,
                layer_velocity_mode: ctx.props().layer_velocity_mode_init,
                program_name: ctx.props().program_name_init.clone(),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TuningFormMessages::PitchPreferenceChange(pitch_preference) => {
                self.tuning.pitch_preference = pitch_preference;
                ctx.props()
                    .on_pitch_preference_change
                    .emit(pitch_preference);
                false
            }
            TuningFormMessages::LayerVelocityModeChange(mode) => {
                self.tuning.layer_velocity_mode = mode;
                ctx.props().on_layer_velocity_mode_change.emit(mode);
                true
            }
            TuningFormMessages::ProgramNameChanged(name) => {
                self.tuning.program_name = name;
                ctx.props()
                    .on_program_name_change
                    .emit(self.tuning.program_name.clone());
                false
            }
            TuningFormMessages::Save => {
                ctx.props().on_save.emit(self.tuning.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let layer_help_text = match self.tuning.layer_velocity_mode {
            LayerVelocityMode::Automatic => {
                "Each layer will only be used for a range of the velocity."
            }
            LayerVelocityMode::Unison => "All the layers will play at the same time.",
        };
        html! {
            <div class="box">
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
                                value={self.tuning.pitch_preference.to_string()}
                                oninput={TuningForm::on_pitch_preference_change(ctx)}
                            />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{"Layer Velocity Mode"}</label>
                        <div class="control">
                            <div class="select">
                                <DropDown<LayerVelocityMode>
                                    initial={ctx.props().layer_velocity_mode_init}
                                    options={vec![LayerVelocityMode::Unison, LayerVelocityMode::Automatic]}
                                    selection_changed={ctx.link().callback(TuningFormMessages::LayerVelocityModeChange)}
                                />
                            </div>
                        </div>
                        <p class="help">{layer_help_text}</p>
                    </div>
                </div>
                <div class="block">
                    <div class="field has-addons">
                        <div class="control">
                            <input
                                class="input"
                                type="text"
                                placeholder="Program Name"
                                oninput={TuningForm::on_program_name_change(ctx)}
                            />
                        </div>
                        <div class="control">
                            <button class="button is-link" onclick={ctx.link().callback(|_: MouseEvent| TuningFormMessages::Save)}>
                                {"Save"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl TuningForm {
    fn on_pitch_preference_change(ctx: &Context<TuningForm>) -> Callback<InputEvent> {
        ctx.link().batch_callback(|e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            Some(TuningFormMessages::PitchPreferenceChange(
                input.value_as_number() as f32,
            ))
        })
    }

    fn on_program_name_change(ctx: &Context<TuningForm>) -> Callback<InputEvent> {
        ctx.link().batch_callback(|e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            Some(TuningFormMessages::ProgramNameChanged(input.value()))
        })
    }
}
