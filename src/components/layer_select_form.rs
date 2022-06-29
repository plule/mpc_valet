use crate::components::LayerSelect;
use crate::model::LayerFile;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::SampleFile;

#[derive(Properties, PartialEq)]
pub struct LayerSelectFormProps {
    #[prop_or_default]
    pub files: Vec<SampleFile>,

    pub on_selected: Callback<Vec<LayerFile>>,

    pub on_cancel: Callback<()>,
}

pub enum LayerSelectFormMessages {
    LayerChanged(usize, usize),
    AllLayerChanged(usize),
    Ok,
    Cancel,
}

/// Select the layer for a list of sample files.
#[derive(Default)]
pub struct LayerSelectForm {
    pub layer_files: Vec<LayerFile>,
    pub all_layers_init: Option<usize>,
}

impl Component for LayerSelectForm {
    type Message = LayerSelectFormMessages;
    type Properties = LayerSelectFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            layer_files: ctx.props().files.iter().map(|f| f.clone().into()).collect(),
            all_layers_init: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LayerSelectFormMessages::LayerChanged(index, layer) => {
                self.layer_files[index].layer = layer;
                self.all_layers_init = None;
                true
            }
            LayerSelectFormMessages::AllLayerChanged(layer) => {
                self.layer_files.iter_mut().for_each(|l| l.layer = layer);
                self.all_layers_init = Some(layer);
                true
            }
            LayerSelectFormMessages::Ok => {
                ctx.props().on_selected.emit(self.layer_files.clone());
                false
            }
            LayerSelectFormMessages::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let samples: Vec<Html> = self
            .layer_files
            .iter()
            .enumerate()
            .map(|(index, sample)| {
                html! {
                    <LayerSelect
                        label={sample.file.clone()}
                        initial={sample.layer}
                        selection_changed={ctx.link().callback(move |layer: usize| LayerSelectFormMessages::LayerChanged(index, layer))}
                    />
                }
            })
            .collect();

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Select layers"}</p>
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Cancel)}></button>
                    </header>
                    <section class="modal-card-body">
                        <div class="tile is-ancestor is-vertical">
                            <LayerSelect
                                label={"All"}
                                initial={self.all_layers_init}
                                selection_changed={ctx.link().callback(LayerSelectFormMessages::AllLayerChanged)}
                            />
                            {samples}
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-success" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Ok)}>{"Ok"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Cancel)}>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.layer_files = ctx.props().files.iter().map(|f| f.clone().into()).collect();
        true
    }
}
