use crate::{hooks::UsersActions, models::{User, UserForm}};
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for the UserRowComponent
#[derive(Properties, PartialEq)]
pub struct UserRowProps {
    pub user: User,
    pub actions: UsersActions,
}

/// Individual user row component
#[function_component(UserRowComponent)]
pub fn user_row_component(props: &UserRowProps) -> Html {
    let is_editing = use_state(|| false);
    let edit_form = use_state(|| UserForm::from_user(&props.user));

    let on_edit_click = {
        let is_editing = is_editing.clone();
        let edit_form = edit_form.clone();
        let user = props.user.clone();
        
        Callback::from(move |_: MouseEvent| {
            is_editing.set(true);
            edit_form.set(UserForm::from_user(&user));
        })
    };

    let on_cancel_click = {
        let is_editing = is_editing.clone();
        
        Callback::from(move |_: MouseEvent| {
            is_editing.set(false);
        })
    };

    let on_save_click = {
        let is_editing = is_editing.clone();
        let edit_form = edit_form.clone();
        let actions = props.actions.clone();
        let user_id = props.user.id.clone();
        
        Callback::from(move |_: MouseEvent| {
            if edit_form.is_valid() {
                let request = edit_form.to_update_request();
                actions.update_user(user_id.clone(), request);
                is_editing.set(false);
            }
        })
    };

    let on_delete_click = {
        let actions = props.actions.clone();
        let user_id = props.user.id.clone();
        
        Callback::from(move |_: MouseEvent| {
            if web_sys::window()
                .unwrap()
                .confirm_with_message("Are you sure you want to delete this user?")
                .unwrap_or(false)
            {
                actions.delete_user(user_id.clone());
            }
        })
    };

    let on_name_change = {
        let edit_form = edit_form.clone();
        
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form_data = (*edit_form).clone();
            form_data.name = input.value();
            edit_form.set(form_data);
        })
    };

    let on_email_change = {
        let edit_form = edit_form.clone();
        
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form_data = (*edit_form).clone();
            form_data.email = input.value();
            edit_form.set(form_data);
        })
    };

    html! {
        <tr class="user-row">
            <td class="user-id">{&props.user.id[..8]}{"..."}</td>
            <td class="user-name">
                {if *is_editing {
                    html! {
                        <input
                            type="text"
                            value={edit_form.name.clone()}
                            oninput={on_name_change}
                            class="edit-input"
                        />
                    }
                } else {
                    html! { &props.user.name }
                }}
            </td>
            <td class="user-email">
                {if *is_editing {
                    html! {
                        <input
                            type="email"
                            value={edit_form.email.clone()}
                            oninput={on_email_change}
                            class="edit-input"
                        />
                    }
                } else {
                    html! { &props.user.email }
                }}
            </td>
            <td class="user-created">{&props.user.created_at[..10]}</td>
            <td class="user-actions">
                {if *is_editing {
                    html! {
                        <div class="action-buttons">
                            <button
                                onclick={on_save_click}
                                class="action-button save"
                                disabled={!edit_form.is_valid()}
                            >
                                {"Save"}
                            </button>
                            <button
                                onclick={on_cancel_click}
                                class="action-button cancel"
                            >
                                {"Cancel"}
                            </button>
                        </div>
                    }
                } else {
                    html! {
                        <div class="action-buttons">
                            <button
                                onclick={on_edit_click}
                                class="action-button edit"
                            >
                                {"Edit"}
                            </button>
                            <button
                                onclick={on_delete_click}
                                class="action-button delete"
                            >
                                {"Delete"}
                            </button>
                        </div>
                    }
                }}
            </td>
        </tr>
    }
}
