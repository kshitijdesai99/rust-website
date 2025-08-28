use crate::{hooks::UsersActions, models::UserForm};
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for the UserFormComponent
#[derive(Properties, PartialEq)]
pub struct UserFormProps {
    pub actions: UsersActions,
}

/// User form component for creating new users
#[function_component(UserFormComponent)]
pub fn user_form_component(props: &UserFormProps) -> Html {
    let form = use_state(UserForm::default);
    let error = use_state(|| None::<String>);

    let on_name_change = {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form_data = (*form).clone();
            form_data.name = input.value();
            form.set(form_data);
        })
    };

    let on_email_change = {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form_data = (*form).clone();
            form_data.email = input.value();
            form.set(form_data);
        })
    };

    let on_submit = {
        let form = form.clone();
        let error = error.clone();
        let actions = props.actions.clone();
        
        Callback::from(move |_: MouseEvent| {
            if !form.is_valid() {
                error.set(Some("Please fill in all fields with valid data".to_string()));
                return;
            }

            let request = form.to_create_request();
            actions.create_user(request);
            
            let mut form_data = (*form).clone();
            form_data.clear();
            form.set(form_data);
            error.set(None);
        })
    };

    html! {
        <div class="user-form">
            <h2>{"Create New User"}</h2>
            
            {if let Some(error_msg) = &*error {
                html! {
                    <div class="error">
                        {error_msg}
                    </div>
                }
            } else {
                html! {}
            }}
            
            <div class="form-group">
                <input
                    type="text"
                    placeholder="Name"
                    value={form.name.clone()}
                    oninput={on_name_change}
                    class="form-input"
                />
                <input
                    type="email"
                    placeholder="Email"
                    value={form.email.clone()}
                    oninput={on_email_change}
                    class="form-input"
                />
                <button
                    onclick={on_submit}
                    class="form-button primary"
                    disabled={!form.is_valid()}
                >
                    {"Create User"}
                </button>
            </div>
        </div>
    }
}
