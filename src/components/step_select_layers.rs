use crate::components::{Icon, LayerSelect};
use crate::model::LayerFile;
use gloo_storage::{LocalStorage, Storage};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::SampleFile;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub files: Vec<SampleFile>,

    #[prop_or_default]
    pub on_next: Callback<Vec<LayerFile>>,

    #[prop_or_default]
    pub on_previous: Callback<()>,
}

pub enum Msg {
    LayerChanged(usize, usize),
    AllLayerChanged(usize),
    Swap(usize, usize),
    Next,
    GuessLayers,
    Previous,
}

/// Select the layer for a list of sample files.
#[derive(Default, Serialize, Deserialize)]
pub struct StepSelectLayers {
    pub layer_files: Vec<LayerFile>,
}

impl From<Vec<SampleFile>> for StepSelectLayers {
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

impl Component for StepSelectLayers {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        LocalStorage::get("layer_select_form").unwrap_or_else(|_| ctx.props().files.clone().into())
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        LocalStorage::delete("layer_select_form");
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let redraw = match msg {
            Msg::LayerChanged(index, layer) => {
                self.layer_files[index].layer = layer;
                true
            }
            Msg::AllLayerChanged(layer) => {
                self.layer_files.iter_mut().for_each(|l| l.layer = layer);
                true
            }
            Msg::Swap(layer1, layer2) => {
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
            Msg::Next => {
                ctx.props().on_next.emit(self.layer_files.clone());
                false
            }
            Msg::Previous => {
                ctx.props().on_previous.emit(());
                false
            }
            Msg::GuessLayers => {
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
                        selection_changed={ctx.link().callback(move |layer: usize| Msg::LayerChanged(index, layer))}
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
            <>
                <LayerSelect
                    label={"All"}
                    initial={all_layers}
                    selection_changed={ctx.link().callback(Msg::AllLayerChanged)}
                />
                {samples}
                <div class="buttons has-addons is-centered">
                    <button class="button" onclick={ctx.link().callback(|_| Msg::GuessLayers)}>
                        <Icon icon="color-wand" text_after="Guess Layers" />
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Swap(0,1))}>
                        <Icon icon="swap-horizontal-outline" text_after="Swap 1-2" />
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Swap(1,2))}>
                        <Icon icon="swap-horizontal-outline" text_after="Swap 2-3" />
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Swap(2,3))}>
                        <Icon icon="swap-horizontal-outline" text_after="Swap 3-4" />
                    </button>
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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        *self = ctx.props().files.clone().into();
        true
    }
}
