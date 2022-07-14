use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("icon".to_string())]
    pub class: String,

    pub icon: String,

    pub text: String,

    #[prop_or_default()]
    pub text_class: String,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    html! {
        <>
            <span class={props.class.clone()}>
                <ion-icon name={props.icon.clone()}></ion-icon>
            </span>
            <span class={props.text_class.clone()}>{props.text.clone()}</span>
        </>
    }
}
