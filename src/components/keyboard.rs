use crate::{model::Keygroup, utils::StaticIterable};
use itertools::Itertools;
use staff::{midi::MidiNote, Pitch};
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub keygroups: Vec<Keygroup>,

    #[prop_or_default]
    pub highlight_keygroup: Option<usize>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &Props) -> Html {
    let keys: Html = MidiNote::iter()
        .map(|note| {
            let midi = (*note).into_byte();
            let mut class: Vec<&str> = vec![match note.pitch() {
                Pitch::CSharp | Pitch::DSharp | Pitch::FSharp | Pitch::GSharp | Pitch::ASharp => {
                    "black"
                }
                Pitch::C | Pitch::F => "white",
                Pitch::D | Pitch::E | Pitch::G | Pitch::A | Pitch::B => "white offset",
            }];

            let mut tooltip = note.to_string();

            if let Some((kg_index, kg)) = props
                .keygroups
                .iter()
                .find_position(|prop| prop.range.contains(&midi))
            {
                let evenness = if kg_index % 2 == 0 { "even" } else { "odd" };
                class.push(evenness);

                if Some(kg_index) == props.highlight_keygroup
                    || kg
                        .layers
                        .iter()
                        .any(|l| l.as_ref().map(|l| l.root == midi).unwrap_or(false))
                {
                    class.push("highlight");
                }

                let samples = kg
                    .layers
                    .iter()
                    .filter_map(|l| l.as_ref())
                    .map(|l| l.file.clone())
                    .join(", ");
                tooltip = format!("{} ({})", tooltip, samples);
            }

            let class = class.join(" ");

            html! {
                <li midi={midi.to_string()} class={class} data-tooltip={tooltip}></li>
            }
        })
        .collect();
    html! {
    <ul id={"keyboard"}>
        {keys}
    </ul>
    }
}
