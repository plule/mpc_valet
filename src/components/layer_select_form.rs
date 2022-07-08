use crate::components::LayerSelect;
use crate::model::LayerFile;
use itertools::Itertools;
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
    Ok,
    Cancel,
}

/// Select the layer for a list of sample files.
#[derive(Default)]
pub struct LayerSelectForm {
    pub layer_files: Vec<LayerFile>,
}

impl Component for LayerSelectForm {
    type Message = LayerSelectFormMessages;
    type Properties = LayerSelectFormProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut s = Self {
            layer_files: vec![],
        };
        s.changed(ctx);
        s
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
                    if f.layer == layer1 {
                        f.layer = layer2;
                    } else if f.layer == layer2 {
                        f.layer = layer1;
                    }
                });
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
                        <p class="modal-card-title">{"Select layers"}</p>
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
                                        <span class="icon">
                                            <ion-icon name="swap-horizontal-outline"></ion-icon>
                                        </span>
                                        <span>{"Swap 1-2"}</span>
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Swap(1,2))}>
                                        <span class="icon">
                                            <ion-icon name="swap-horizontal-outline"></ion-icon>
                                        </span>
                                        <span>{"Swap 2-3"}</span>
                                    </button>
                                    <button class="button" onclick={ctx.link().callback(|_| LayerSelectFormMessages::Swap(2,3))}>
                                        <span class="icon">
                                            <ion-icon name="swap-horizontal-outline"></ion-icon>
                                        </span>
                                        <span>{"Swap 3-4"}</span>
                                    </button>
                                </div>
                            </div>
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
        // Initiate the list of layer files from the list of files with roots
        self.layer_files = ctx
            .props()
            .files
            .iter()
            // Sort by root note (group_by needs it)
            .sorted_by(|a, b| a.root.cmp(&b.root))
            // group by root note
            .group_by(|f| f.root.into_byte())
            .into_iter()
            .map(|(_, group)| {
                group
                    // Sort each note with the same root per file name
                    .sorted_by(|a, b| a.file.cmp(&b.file))
                    .enumerate()
                    // Assign a different layer to each note with the same root,
                    // based on the sample alphabetical order
                    .map(|(index, file)| LayerFile::from_sample_file(file.clone(), index % 4))
            })
            .flatten()
            .sorted_by(|a, b| a.file.cmp(&b.file))
            .collect();
        true
    }
}
