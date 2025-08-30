use yew::prelude::*;
use yew_router::prelude::*;

pub mod app_routes;
pub mod pages;
pub mod config;
pub mod services;

use app_routes::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
