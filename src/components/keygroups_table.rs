use crate::model::Keygroup;

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
            let range = format!(
                "{}{} to {}{}",
                kg.range.start().pitch(),
                kg.range.start().octave(),
                kg.range.end().pitch(),
                kg.range.end().octave(),
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
