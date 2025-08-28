use frontend::{
    components::{UserFormComponent, UserListComponent},
    hooks::use_users,
};
use yew::prelude::*;

/// Main application component
#[function_component(App)]
fn app() -> Html {
    let (users_state, users_actions) = use_users();

    // Load users on component mount
    use_effect_with((), {
        let actions = users_actions.clone();
        move |_| {
            actions.load_users();
            || ()
        }
    });

    // Clear error when user interacts
    let on_clear_error = {
        let actions = users_actions.clone();
        Callback::from(move |_: MouseEvent| {
            actions.clear_error();
        })
    };

    html! {
        <div class="app">
            <div class="container">
                <header class="app-header">
                    <h1>{"User Admin Panel"}</h1>
                </header>
                
                <main class="app-main">
                    {if let Some(error) = &users_state.error {
                        html! {
                            <div class="error-banner">
                                <div class="error-content">
                                    <span class="error-message">{error}</span>
                                    <button 
                                        class="error-close"
                                        onclick={on_clear_error}
                                    >
                                        {"×"}
                                    </button>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    
                    <section class="form-section">
                        <UserFormComponent actions={users_actions.clone()} />
                    </section>
                    
                    <section class="list-section">
                        <UserListComponent 
                            users={users_state.users.clone()}
                            loading={users_state.loading}
                            actions={users_actions.clone()}
                        />
                    </section>
                </main>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
