use crate::components::Keyboard;
use crate::components::KeygroupsTable;
use crate::components::{LayerSelectForm, RootNotesForm, TuningForm};
use crate::model::{KeygroupProgram, LayerFile, LayerVelocityMode, SampleFile};
use anyhow::bail;
use js_sys::encode_uri_component;
use log::debug;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::Tuning;

pub enum Msg {
    FilesDropped(Vec<String>),
    RootNoteSelected(Vec<SampleFile>),
    LayersSelected(Vec<LayerFile>),
    ClearDroppedFiles,
    PitchPreference(f32),
    Save(Tuning),
    HighlightKeygroup(Option<usize>),
}

/// Possible stages when adding files
pub enum FileAddition {
    /// No change is going on
    Empty,

    /// A list of file were dropped
    FileList(Vec<String>),

    /// The file roots were selected
    SampleList(Vec<SampleFile>),
}

/// Main component: Create the keygroup programs.
pub struct KeygroupCreator {
    /// Keygroup program being built.
    program: KeygroupProgram,

    /// Pitch preference (0 to 1, prefer pitching down or up)
    pitch_preference: f32,

    /// Files currently being processed
    dropped_files: FileAddition,

    /// Keygroup index to highlight
    highlight_keygroup: Option<usize>,
}

impl Component for KeygroupCreator {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            program: KeygroupProgram::default(),
            pitch_preference: 0.5,
            dropped_files: FileAddition::Empty,
            highlight_keygroup: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FilesDropped(files) => {
                self.dropped_files = FileAddition::FileList(files);
                true
            }
            Msg::RootNoteSelected(samples) => {
                self.dropped_files = FileAddition::SampleList(samples);
                true
            }
            Msg::LayersSelected(samples) => {
                self.program.insert_layer_files(samples);
                self.program.sort_keygroups();
                self.program.guess_ranges(self.pitch_preference);
                self.dropped_files = FileAddition::Empty;
                true
            }
            Msg::ClearDroppedFiles => {
                self.dropped_files = FileAddition::Empty;
                true
            }
            Msg::PitchPreference(pitch_preference) => {
                self.pitch_preference = pitch_preference;
                self.program.guess_ranges(pitch_preference);
                true
            }
            Msg::Save(tuning) => {
                self.program.name = tuning.program_name;
                self.program
                    .set_velocity_layer_mode(&tuning.layer_velocity_mode);
                self.export().unwrap();
                false
            }
            Msg::HighlightKeygroup(index) => {
                self.highlight_keygroup = index;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        debug!("Redrawing main view");
        html! {
        <div id="drop_zone" ondrop={KeygroupCreator::on_file_drop(ctx)} ondragover={|e: DragEvent| e.prevent_default()}>
            <div class="container">
                <div class="box">
                    <KeygroupsTable keygroups={self.program.keygroups.clone()} on_hovered_kg={ctx.link().callback(Msg::HighlightKeygroup)} />
                    {self.view_file_addition(ctx)}
                    <div class="file is-boxed is-centered">
                        <label class="file-label">
                            <input class="file-input" type="file" name="samples" multiple=true accept=".wav" oninput={KeygroupCreator::on_file_input(ctx)} />
                            <span class="file-cta">
                                <span class="file-icon">
                                    <i class="fas fa-upload"></i>
                                </span>
                                <span class="file-label">
                                    {"Add Samples..."}
                                </span>
                            </span>
                        </label>
                    </div>
                </div>
                <Keyboard keygroups={self.program.keygroups.clone()} highlight_keygroup={self.highlight_keygroup} />
                <TuningForm
                    pitch_preference_init=0.5
                    layer_velocity_mode_init={LayerVelocityMode::Spread}
                    on_pitch_preference_change={ctx.link().callback(Msg::PitchPreference)}
                    on_save={ctx.link().callback(Msg::Save)}
                />
            </div>
        </div>
        }
    }
}

impl KeygroupCreator {
    fn view_file_addition(&self, ctx: &Context<Self>) -> Html {
        match &self.dropped_files {
            FileAddition::Empty => html! {},
            FileAddition::FileList(files) => html! {
                <RootNotesForm
                    files={files.clone()}
                    on_selected={ctx.link().callback(Msg::RootNoteSelected)}
                    on_cancel={ctx.link().callback(|_| Msg::ClearDroppedFiles)}
                />
            },
            FileAddition::SampleList(files) => html! {
                <LayerSelectForm
                    files={files.clone()}
                    on_selected={ctx.link().callback(Msg::LayersSelected)}
                    on_cancel={ctx.link().callback(|_| Msg::ClearDroppedFiles)}
                />
            },
        }
    }

    fn on_file_input(ctx: &Context<KeygroupCreator>) -> Callback<InputEvent> {
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

    fn on_file_drop(ctx: &Context<KeygroupCreator>) -> Callback<DragEvent> {
        ctx.link().batch_callback(move |e: DragEvent| {
            e.prevent_default();
            let files = e.data_transfer()?.files()?;
            let file_names: Vec<String> = (0..files.length())
                .filter_map(|i| Some(files.get(i)?.name()))
                .collect();
            Some(Msg::FilesDropped(file_names))
        })
    }

    fn export(&self) -> anyhow::Result<()> {
        use anyhow::Context;
        let mut file_content = Vec::<u8>::new();
        self.program.export(&mut file_content)?;
        let file_content = String::from_utf8(file_content)
            .context("Failed to convert the instrument file to UTF8")?;
        let file_content = encode_uri_component(&file_content);
        let file_name = format!("{}.xpm", self.program.name);

        let window = web_sys::window().context("Failed to get the browser window")?;
        let document = window
            .document()
            .context("Failed to get the window document")?;
        let body = document.body().context("Failed to get the document body")?;
        let element = document
            .create_element("a")
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to insert a link in the document")?;
        let element = element
            .dyn_into::<web_sys::HtmlElement>()
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to convert the element to an HTML element")?;
        element
            .set_attribute(
                "href",
                format!("data:text/plain;charset=utf-8,{}", file_content).as_str(),
            )
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to set the element destination")?;
        element
            .set_attribute("download", &file_name)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to create the download file name")?;
        body.append_child(&element)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to insert the element in the document")?;
        element.click();
        body.remove_child(&element)
            .or_else(|e| bail!(e.as_string().unwrap_or_default()))
            .context("Failed to remove the element from the document")?;
        Ok(())
    }
}