//use crate::components::RangeSlider;
use crate::model::{LayerVelocityMode, LayerVelocityModeSelection};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_utils::components::drop_down::DropDown;

#[derive(Properties, PartialEq)]
pub struct TuningFormProps {
    #[prop_or_default]
    pub pitch_preference: f32,

    #[prop_or_default]
    pub layer_velocity_mode: LayerVelocityMode,

    #[prop_or_default]
    pub program_name: String,

    #[prop_or_default]
    pub on_pitch_preference_change: Callback<f32>,

    #[prop_or_default]
    pub on_layer_velocity_mode_change: Callback<LayerVelocityMode>,

    #[prop_or_default]
    pub on_program_name_change: Callback<String>,

    #[prop_or_default]
    pub on_save: Callback<()>,
}

pub enum TuningFormMessages {
    PitchPreferenceChange(f32),
    LayerVelocityModeChange(LayerVelocityMode),
    ProgramNameChanged(String),
    Save,
}

pub enum RangeValue {
    Start,
    End,
}

/// Root note selector for a list of sample files.
#[derive(Default)]
pub struct TuningForm {}

impl Component for TuningForm {
    type Message = TuningFormMessages;
    type Properties = TuningFormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TuningFormMessages::PitchPreferenceChange(pitch_preference) => {
                ctx.props()
                    .on_pitch_preference_change
                    .emit(pitch_preference);
                false
            }
            TuningFormMessages::LayerVelocityModeChange(mode) => {
                ctx.props().on_layer_velocity_mode_change.emit(mode);
                false
            }
            TuningFormMessages::ProgramNameChanged(name) => {
                ctx.props().on_program_name_change.emit(name);
                false
            }
            TuningFormMessages::Save => {
                ctx.props().on_save.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let layer_help_text = match ctx.props().layer_velocity_mode {
            LayerVelocityMode::Automatic => {
                "Each layer will only be used for a range of the velocity."
            }
            LayerVelocityMode::Unison => "All the layers will play at the same time.",
            LayerVelocityMode::Manual(_) => "The ranges are manually entered.",
        };

        let layer_velocity_mode_selection: LayerVelocityModeSelection =
            ctx.props().layer_velocity_mode.clone().into();

        let mut layer_velocity_mode_html: Html = html! {
            <div class="field">
                <label class="label">{"Layer Velocity Mode"}</label>
                <div class="control">
                    <div class="select">
                        <DropDown<LayerVelocityModeSelection>
                            initial={layer_velocity_mode_selection}
                            options={vec![LayerVelocityModeSelection::Unison, LayerVelocityModeSelection::Automatic]}
                            selection_changed={ctx.link().callback(|mode: LayerVelocityModeSelection| TuningFormMessages::LayerVelocityModeChange(mode.into()))}
                        />
                    </div>
                </div>
                <p class="help">{layer_help_text}</p>
            </div>
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
                                value={ctx.props().pitch_preference.to_string()}
                                oninput={TuningForm::on_pitch_preference_change(ctx)}
                            />
                        </div>
                    </div>
                    {layer_velocity_mode_html}
                </div>
                <div class="block">
                    <div class="field has-addons">
                        <div class="control">
                            <input
                                class="input"
                                type="text"
                                placeholder="Program Name"
                                value={ctx.props().program_name.clone()}
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

    fn on_manual_layer_change(
        ctx: &Context<TuningForm>,
        layer: usize,
        range_value: RangeValue,
    ) -> Callback<InputEvent> {
        let velocity_mode = ctx.props().layer_velocity_mode.clone();
        ctx.link().batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            let value = input.value_as_number() as u8;
            if let LayerVelocityMode::Manual(ranges) = &velocity_mode {
                let mut ranges = ranges.clone();
                ranges[layer] = match range_value {
                    RangeValue::Start => value..=ranges[layer].end().clone(),
                    RangeValue::End => ranges[layer].start().clone()..=value,
                };

                return Some(TuningFormMessages::LayerVelocityModeChange(
                    LayerVelocityMode::Manual(ranges),
                ));
            }
            None
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
