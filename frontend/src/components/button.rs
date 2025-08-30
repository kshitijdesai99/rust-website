use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub class: String,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let handle_click = Callback::from(move |_| onclick.emit(()));

    html! {
        <button 
            class={format!("btn {}", props.class)}
            onclick={handle_click}
            disabled={props.disabled}
        >
            { &props.text }
        </button>
    }
}
