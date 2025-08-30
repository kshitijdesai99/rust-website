use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub placeholder: String,
    pub value: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub input_type: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let onchange = props.onchange.clone();
    
    let handle_change = Callback::from(move |e: Event| {
        let target = e.target_unchecked_into::<HtmlInputElement>();
        let value = target.value();
        onchange.emit(value);
    });

    let input_type = if props.input_type.is_empty() {
        "text".to_string()
    } else {
        props.input_type.clone()
    };

    html! {
        <div class="input-group">
            <input
                type={input_type}
                placeholder={props.placeholder.clone()}
                value={props.value.clone()}
                onchange={handle_change}
                class="form-input"
            />
        </div>
    }
}
