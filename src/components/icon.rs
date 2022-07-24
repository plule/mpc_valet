use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or("icon".to_string())]
    pub class: String,

    pub icon: String,

    #[prop_or(None)]
    pub text_before: Option<String>,

    #[prop_or(None)]
    pub text_after: Option<String>,

    #[prop_or_default()]
    pub text_class: String,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let Props {
        class,
        icon,
        text_before,
        text_after,
        text_class,
    } = &(*props).clone();

    let text_before = if let Some(text) = text_before {
        html! {
            <span class={text_class}>{text}</span>
        }
    } else {
        html! {}
    };

    let text_after = if let Some(text) = text_after {
        html! {
            <span class={text_class}>{text}</span>
        }
    } else {
        html! {}
    };

    html! {
        <>
            {text_before}
            <span class={class}>
                <ion-icon name={icon.clone()}></ion-icon>
            </span>
            {text_after}
        </>
    }
}
