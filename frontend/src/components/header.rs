use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <nav class="navbar">
                <div class="navbar-brand">
                    <h2>{ "My Rust Website" }</h2>
                </div>
                <ul class="navbar-menu">
                    <li><a href="/">{ "Home" }</a></li>
                    <li><a href="/about">{ "About" }</a></li>
                    <li><a href="/contact">{ "Contact" }</a></li>
                </ul>
            </nav>
        </header>
    }
}
