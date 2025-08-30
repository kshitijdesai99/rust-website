use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-content">
                <p>{ "© 2024 My Rust Website. Built with Yew and Rust." }</p>
                <div class="footer-links">
                    <a href="/privacy">{ "Privacy" }</a>
                    <a href="/terms">{ "Terms" }</a>
                </div>
            </div>
        </footer>
    }
}
