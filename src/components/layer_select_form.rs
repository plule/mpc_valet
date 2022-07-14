use crate::components::{Icon, LayerSelect};
use crate::model::LayerFile;
use gloo_storage::{LocalStorage, Storage};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
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
    Swap(usize, usize),
    Done,
    Reset,
    Cancel,
}

/// Select the layer for a list of sample files.
#[derive(Default, Serialize, Deserialize)]
pub struct LayerSelectForm {
    pub layer_files: Vec<LayerFile>,
}

impl From<Vec<SampleFile>> for LayerSelectForm {
    fn from(sample_files: Vec<SampleFile>) -> Self {
        // Initiate the list of layer files from the list of files with roots
        let layer_files: Vec<LayerFile> = sample_files
            .iter()
            // Sort by root note (group_by needs it)
            .sorted_by(|a, b| a.root.cmp(&b.root))
            // group by root note
            .group_by(|f| f.root)
            .into_iter()
            .flat_map(|(_, group)| {
                group
                    // Sort each note with the same root per file name
                    .sorted_by(|a, b| a.file.cmp(&b.file))
                    .enumerate()
                    // Assign a different layer to each note with the same root,
                    // based on the sample alphabetical order
                    .map(|(index, file)| LayerFile::from_sample_file(file.clone(), index % 4))
            })
            .sorted_by(|a, b| a.file.cmp(&b.file))
            .collect();
        Self { layer_files }
    }
}

impl Component for LayerSelectForm {
    type Message = LayerSelectFormMessages;
    type Properties = LayerSelectFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        LocalStorage::get("layer_select_form").unwrap_or_else(|_| ctx.props().files.clone().into())
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("layer_select_form");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            LayerSelectFormMessages::LayerChanged(index, layer) => {
                self.layer_files[index].layer = layer;
                true
            }
            LayerSelectFormMessages::AllLayerChanged(layer) => {
                self.layer_files.iter_mut().for_each(|l| l.layer = layer);
                true
            }
            LayerSelectFormMessages::Swap(layer1, layer2) => {
                self.layer_files.iter_mut().for_each(|f| {
                    f.layer = if f.layer == layer1 {
                        layer2
                    } else if f.layer == layer2 {
                        layer1
                    } else {
                        f.layer
                    };
                });
                true
            }
            LayerSelectFormMessages::Done => {
                ctx.props().on_selected.emit(self.layer_files.clone());
                false
            }

            LayerSelectFormMessages::Cancel => {
                ctx.props().on_cancel.emit(());
                false
            }
            LayerSelectFormMessages::Reset => {
                *self = ctx.props().files.clone().into();
                true
            }
        };
        LocalStorage::set("layer_select_form", self).unwrap_or_else(|e| {
            log::error!("{e}");
        });
        redraw
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

        let used_layers: Vec<usize> = self
            .layer_files
            .iter()
            .map(|f| f.layer)
            .unique()
            .collect_vec();

        let all_layers = match used_layers.len() {
            1 => Some(used_layers[0]),
            _ => None,
        };

        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <Icon icon="layers" text="Select layers" text_class="modal-card-title" />
                        <button class="delete" aria-label="close" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Cancel)}></button>
                    </header>
                    <section class="modal-card-body">
                        <LayerSelect
                            label={"All"}
                            initial={all_layers}
                            selection_changed={ctx.link().callback(LayerSelectFormMessages::AllLayerChanged)}
                        />
                        {samples}
                        <div class="columns">
                            <div class="column is-one-quarter">
                                {"Swap Layers"}
                            </div>
                            <div class="column">
                                <div class="buttons has-addons is-centered">
                                    <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Swap(0,1))}>
                                        <Icon icon="swap-horizontal-outline" text="Swap 1-2" />
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Swap(1,2))}>
                                        <Icon icon="swap-horizontal-outline" text="Swap 2-3" />
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Swap(2,3))}>
                                        <Icon icon="swap-horizontal-outline" text="Swap 3-4" />
                                    </button>
                                </div>
                            </div>
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        <div class="buttons has-addons">
                            <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Cancel)}>
                                <Icon icon="trash" text ="Cancel" />
                            </button>
                            <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Reset)}>
                                <Icon icon="refresh" text="Reset" />
                            </button>
                            <button class="button is-success" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Done)}>{"Ok"}</button>
                        </div>
                    </footer>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        *self = ctx.props().files.clone().into();
        true
    }
}
