use crate::model::Keygroup;

use music_note::midi::MidiNote;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub keygroups: Vec<Keygroup>,

    #[prop_or_default]
    pub on_hovered_kg: Callback<Option<usize>>,
}

#[function_component(KeygroupsTable)]
pub fn keygroups_table(props: &Props) -> Html {
    if props.keygroups.is_empty() {
        return html! {};
    }

    let keygroup_rows: Html = props
        .keygroups
        .iter()
        .enumerate()
        .map(|(index, kg)| {
            let start = MidiNote::from_byte(*kg.range.start());
            let end = MidiNote::from_byte(*kg.range.end());
            let range = format!(
                "{}{} to {}{}",
                start.pitch(),
                start.octave(),
                end.pitch(),
                end.octave(),
            );

            let layer_cells: Html = kg
                .layers
                .iter()
                .map(|layer| {
                    if let Some(layer) = layer {
                        return html! {
                            <td>
                                {layer.file.to_string()}
                            </td>
                        };
                    }
                    html! {<td/>}
                })
                .collect();

            let hovered = props.on_hovered_kg.clone();
            html! {
                <tr onmouseover={move |_|{hovered.emit(Some(index))}}>
                    <td><strong>{range}</strong></td>
                    {layer_cells}
                </tr>
            }
        })
        .collect();

    let hovered = props.on_hovered_kg.clone();
    html! {
        <table class="table is-fullwidth is-hoverable is-striped" onmouseout={move |_|{hovered.emit(None)}}>
            <thead>
                <tr>
                    <th>{"Range"}</th>
                    <th>{"Layer 1"}</th>
                    <th>{"Layer 2"}</th>
                    <th>{"Layer 3"}</th>
                    <th>{"Layer 4"}</th>
                </tr>
            </thead>
            <tbody>
                {keygroup_rows}
            </tbody>
        </table>
    }
}
