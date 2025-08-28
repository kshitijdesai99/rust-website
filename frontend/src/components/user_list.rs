use crate::{
    components::UserRowComponent,
    hooks::{UsersActions},
    models::User,
};
use yew::prelude::*;

/// Properties for the UserListComponent
#[derive(Properties, PartialEq)]
pub struct UserListProps {
    pub users: Vec<User>,
    pub loading: bool,
    pub actions: UsersActions,
}

/// User list component displaying all users in a table
#[function_component(UserListComponent)]
pub fn user_list_component(props: &UserListProps) -> Html {
    html! {
        <div class="user-list">
            <h2>{"Users"}</h2>
            
            {if props.loading {
                html! {
                    <div class="loading">
                        {"Loading users..."}
                    </div>
                }
            } else if props.users.is_empty() {
                html! {
                    <div class="empty-state">
                        {"No users found. Create your first user above!"}
                    </div>
                }
            } else {
                html! {
                    <div class="table-container">
                        <table class="users-table">
                            <thead>
                                <tr>
                                    <th>{"ID"}</th>
                                    <th>{"Name"}</th>
                                    <th>{"Email"}</th>
                                    <th>{"Created"}</th>
                                    <th>{"Actions"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for props.users.iter().map(|user| {
                                    html! {
                                        <UserRowComponent
                                            key={user.id.clone()}
                                            user={user.clone()}
                                            actions={props.actions.clone()}
                                        />
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                }
            }}
        </div>
    }
}
