use yew::prelude::*;
use yew_router::prelude::*;

/// Application routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/user-admin")]
    UserAdmin,
}

/// Home page component
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home">
            <div class="container">
                <header class="app-header">
                    <h1>{"Welcome to User Management System"}</h1>
                </header>

                <main class="app-main">
                    <div class="hero-section">
                        <h2>{"Manage Your Users Efficiently"}</h2>
                        <p>{"Access the user administration panel to create, view, update, and delete users."}</p>

                        <div class="navigation">
                            <Link<Route> to={Route::UserAdmin} classes="nav-button primary">
                                {"Go to User Admin"}
                            </Link<Route>>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    }
}

/// User admin page component
#[function_component(UserAdmin)]
pub fn user_admin() -> Html {
    html! {
        <div class="user-admin">
            <div class="container">
                <header class="app-header">
                    <div class="header-content">
                        <Link<Route> to={Route::Home} classes="back-button">
                            {"← Back to Home"}
                        </Link<Route>>
                        <h1>{"User Admin Panel"}</h1>
                    </div>
                </header>

                <main class="app-main">
                    <UserAdminContent />
                </main>
            </div>
        </div>
    }
}

/// User admin content component (extracted from original App)
#[function_component(UserAdminContent)]
fn user_admin_content() -> Html {
    let (users_state, users_actions) = crate::hooks::use_users();

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
        <>
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
                <crate::components::UserFormComponent actions={users_actions.clone()} />
            </section>

            <section class="list-section">
                <crate::components::UserListComponent
                    users={users_state.users.clone()}
                    loading={users_state.loading}
                    actions={users_actions.clone()}
                />
            </section>
        </>
    }
}
