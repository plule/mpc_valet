use crate::{components::Icon, model::KeygroupProgram};
use anyhow::bail;
use gloo_storage::{LocalStorage, Storage};
use js_sys::encode_uri_component;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{html, Callback, Component, Context, Html, Properties};

#[derive(Default, Serialize, Deserialize)]
pub struct StepDone {
    /// The keygroup program ready to be saved
    program: KeygroupProgram,
}

pub enum Msg {
    Previous,
    ProgramNameChanged(String),
    Save,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub program: KeygroupProgram,

    #[prop_or_default]
    pub on_previous: Callback<()>,
}

impl Component for StepDone {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        LocalStorage::get("step_done").unwrap_or_else(|_| Self {
            program: ctx.props().program.clone(),
        })
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("step_done");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            Msg::ProgramNameChanged(name) => {
                self.program.name = name.clone();
                true
            }
            Msg::Save => {
                if let Err(e) = self.export() {
                    log::error!("{}", e);
                };
                true
            }
            Msg::Previous => {
                ctx.props().on_previous.emit(());
                true
            }
        };
        LocalStorage::set("step_done", self).unwrap_or_else(|e| {
            log::error!("{e}");
        });
        redraw
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="block">
                <div class="field has-addons">
                    <div class="control">
                        <input
                            class="input"
                            type="text"
                            placeholder="Program Name"
                            value={self.program.name.clone()}
                            oninput={StepDone::on_program_name_change(ctx)}
                        />
                    </div>
                    <div class="control">
                        <button class="button is-link" onclick={ctx.link().callback(|_: MouseEvent| Msg::Save)}>
                            <Icon icon="save" text_after="Save" />
                        </button>
                    </div>
                </div>
                /*<div class="buttons has-addons is-centered">
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Previous)}>
                        <Icon icon="caret-back" text_after ="Previous" />
                    </button>
                </div>*/
            </div>
        }
    }
}

impl StepDone {
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

    fn on_program_name_change(ctx: &Context<StepDone>) -> Callback<InputEvent> {
        ctx.link().batch_callback(|e: InputEvent| {
            let input: HtmlInputElement = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
            Some(Msg::ProgramNameChanged(input.value()))
        })
    }
}
