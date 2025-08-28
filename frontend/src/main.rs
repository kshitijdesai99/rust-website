use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
struct Hello {
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let msg = use_state(|| String::new());

    let onclick = {
        let msg = msg.clone();
        Callback::from(move |_| {
            let msg = msg.clone();
            spawn_local(async move {
                if let Ok(resp) = Request::get("http://127.0.0.1:3000/api/hello").send().await {
                    if let Ok(data) = resp.json::<Hello>().await {
                        msg.set(data.message);
                    } else {
                        msg.set("Parse error".into());
                    }
                } else {
                    msg.set("Request error".into());
                }
            });
        })
    };

    html! {
        <div style="font-family: system-ui, -apple-system, Segoe UI, Roboto; padding: 2rem;">
            <h1>{ "Yew Frontend" }</h1>
            <button {onclick}>{ "Call Backend" }</button>
            <p>{ (*msg).clone() }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
