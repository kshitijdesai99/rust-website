use frontend::{
    routes::{Home, Route, UserAdmin},
};
use yew::prelude::*;
use yew_router::prelude::*;

/// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// Route switcher
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::UserAdmin => html! { <UserAdmin /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
