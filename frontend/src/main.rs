use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    id: String,
    name: String,
    email: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

enum Msg {
    LoadUsers,
    UsersLoaded(Vec<User>),
    CreateUser,
    UserCreated(User),
    UpdateUser(String),
    UserUpdated(User),
    DeleteUser(String),
    UserDeleted(String),
    SetName(String),
    SetEmail(String),
    SetEditMode(Option<String>),
    SetEditName(String),
    SetEditEmail(String),
    Error(String),
}

struct App {
    users: Vec<User>,
    loading: bool,
    name: String,
    email: String,
    edit_mode: Option<String>,
    edit_name: String,
    edit_email: String,
    error: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadUsers);
        Self {
            users: vec![],
            loading: false,
            name: String::new(),
            email: String::new(),
            edit_mode: None,
            edit_name: String::new(),
            edit_email: String::new(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadUsers => {
                self.loading = true;
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("http://127.0.0.1:3000/api/users").send().await {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<Vec<User>>().await {
                                    Ok(users) => link.send_message(Msg::UsersLoaded(users)),
                                    Err(e) => link.send_message(Msg::Error(format!("Parse error: {}", e))),
                                }
                            } else {
                                link.send_message(Msg::Error("Failed to load users".to_string()));
                            }
                        }
                        Err(e) => link.send_message(Msg::Error(format!("Request error: {}", e))),
                    }
                });
                true
            }
            Msg::UsersLoaded(users) => {
                self.users = users;
                self.loading = false;
                self.error = None;
                true
            }
            Msg::CreateUser => {
                if self.name.is_empty() || self.email.is_empty() {
                    self.error = Some("Name and email are required".to_string());
                    return true;
                }
                
                let create_user = CreateUser {
                    name: self.name.clone(),
                    email: self.email.clone(),
                };
                
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::post("http://127.0.0.1:3000/api/users")
                        .json(&create_user)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<User>().await {
                                    Ok(user) => link.send_message(Msg::UserCreated(user)),
                                    Err(e) => link.send_message(Msg::Error(format!("Parse error: {}", e))),
                                }
                            } else {
                                link.send_message(Msg::Error("Failed to create user".to_string()));
                            }
                        }
                        Err(e) => link.send_message(Msg::Error(format!("Request error: {}", e))),
                    }
                });
                true
            }
            Msg::UserCreated(user) => {
                self.users.push(user);
                self.name.clear();
                self.email.clear();
                self.error = None;
                true
            }
            Msg::UpdateUser(id) => {
                let update_user = UpdateUser {
                    name: if self.edit_name.is_empty() { None } else { Some(self.edit_name.clone()) },
                    email: if self.edit_email.is_empty() { None } else { Some(self.edit_email.clone()) },
                };
                
                let link = ctx.link().clone();
                let url = format!("http://127.0.0.1:3000/api/users/{}", id);
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::put(&url)
                        .json(&update_user)
                        .unwrap()
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<User>().await {
                                    Ok(user) => link.send_message(Msg::UserUpdated(user)),
                                    Err(e) => link.send_message(Msg::Error(format!("Parse error: {}", e))),
                                }
                            } else {
                                link.send_message(Msg::Error("Failed to update user".to_string()));
                            }
                        }
                        Err(e) => link.send_message(Msg::Error(format!("Request error: {}", e))),
                    }
                });
                true
            }
            Msg::UserUpdated(updated_user) => {
                if let Some(index) = self.users.iter().position(|u| u.id == updated_user.id) {
                    self.users[index] = updated_user;
                }
                self.edit_mode = None;
                self.edit_name.clear();
                self.edit_email.clear();
                self.error = None;
                true
            }
            Msg::DeleteUser(id) => {
                let link = ctx.link().clone();
                let url = format!("http://127.0.0.1:3000/api/users/{}", id.clone());
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::delete(&url).send().await {
                        Ok(response) => {
                            if response.ok() {
                                link.send_message(Msg::UserDeleted(id));
                            } else {
                                link.send_message(Msg::Error("Failed to delete user".to_string()));
                            }
                        }
                        Err(e) => link.send_message(Msg::Error(format!("Request error: {}", e))),
                    }
                });
                true
            }
            Msg::UserDeleted(id) => {
                self.users.retain(|u| u.id != id);
                self.error = None;
                true
            }
            Msg::SetName(name) => {
                self.name = name;
                true
            }
            Msg::SetEmail(email) => {
                self.email = email;
                true
            }
            Msg::SetEditMode(user_id) => {
                if let Some(id) = &user_id {
                    if let Some(user) = self.users.iter().find(|u| &u.id == id) {
                        self.edit_name = user.name.clone();
                        self.edit_email = user.email.clone();
                    }
                }
                self.edit_mode = user_id;
                true
            }
            Msg::SetEditName(name) => {
                self.edit_name = name;
                true
            }
            Msg::SetEditEmail(email) => {
                self.edit_email = email;
                true
            }
            Msg::Error(error) => {
                self.error = Some(error);
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="padding: 20px; font-family: Arial, sans-serif;">
                <h1>{"User Admin Panel"}</h1>
                
                {if let Some(error) = &self.error {
                    html! {
                        <div style="color: red; margin-bottom: 20px; padding: 10px; border: 1px solid red; border-radius: 4px;">
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                <div style="margin-bottom: 30px; padding: 20px; border: 1px solid #ccc; border-radius: 4px;">
                    <h2>{"Create New User"}</h2>
                    <div style="margin-bottom: 10px;">
                        <input
                            type="text"
                            placeholder="Name"
                            value={self.name.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::SetName(input.value())
                            })}
                            style="padding: 8px; margin-right: 10px; border: 1px solid #ccc; border-radius: 4px;"
                        />
                        <input
                            type="email"
                            placeholder="Email"
                            value={self.email.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::SetEmail(input.value())
                            })}
                            style="padding: 8px; margin-right: 10px; border: 1px solid #ccc; border-radius: 4px;"
                        />
                        <button
                            onclick={ctx.link().callback(|_| Msg::CreateUser)}
                            style="padding: 8px 16px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            {"Create User"}
                        </button>
                    </div>
                </div>

                <div>
                    <h2>{"Users"}</h2>
                    {if self.loading {
                        html! { <p>{"Loading..."}</p> }
                    } else {
                        html! {
                            <table style="width: 100%; border-collapse: collapse;">
                                <thead>
                                    <tr style="background-color: #f8f9fa;">
                                        <th style="padding: 12px; border: 1px solid #ddd; text-align: left;">{"ID"}</th>
                                        <th style="padding: 12px; border: 1px solid #ddd; text-align: left;">{"Name"}</th>
                                        <th style="padding: 12px; border: 1px solid #ddd; text-align: left;">{"Email"}</th>
                                        <th style="padding: 12px; border: 1px solid #ddd; text-align: left;">{"Created"}</th>
                                        <th style="padding: 12px; border: 1px solid #ddd; text-align: left;">{"Actions"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {for self.users.iter().map(|user| self.view_user_row(ctx, user))}
                                </tbody>
                            </table>
                        }
                    }}
                </div>
            </div>
        }
    }
}

impl App {
    fn view_user_row(&self, ctx: &Context<Self>, user: &User) -> Html {
        let user_id = user.id.clone();
        let is_editing = self.edit_mode.as_ref() == Some(&user.id);

        html! {
            <tr>
                <td style="padding: 12px; border: 1px solid #ddd;">{&user.id[..8]}{"..."}</td>
                <td style="padding: 12px; border: 1px solid #ddd;">
                    {if is_editing {
                        html! {
                            <input
                                type="text"
                                value={self.edit_name.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetEditName(input.value())
                                })}
                                style="width: 100%; padding: 4px; border: 1px solid #ccc;"
                            />
                        }
                    } else {
                        html! { &user.name }
                    }}
                </td>
                <td style="padding: 12px; border: 1px solid #ddd;">
                    {if is_editing {
                        html! {
                            <input
                                type="email"
                                value={self.edit_email.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::SetEditEmail(input.value())
                                })}
                                style="width: 100%; padding: 4px; border: 1px solid #ccc;"
                            />
                        }
                    } else {
                        html! { &user.email }
                    }}
                </td>
                <td style="padding: 12px; border: 1px solid #ddd;">{&user.created_at[..10]}</td>
                <td style="padding: 12px; border: 1px solid #ddd;">
                    {if is_editing {
                        html! {
                            <>
                                <button
                                    onclick={ctx.link().callback({
                                        let id = user_id.clone();
                                        move |_| Msg::UpdateUser(id.clone())
                                    })}
                                    style="padding: 4px 8px; margin-right: 5px; background-color: #28a745; color: white; border: none; border-radius: 2px; cursor: pointer;"
                                >
                                    {"Save"}
                                </button>
                                <button
                                    onclick={ctx.link().callback(|_| Msg::SetEditMode(None))}
                                    style="padding: 4px 8px; background-color: #6c757d; color: white; border: none; border-radius: 2px; cursor: pointer;"
                                >
                                    {"Cancel"}
                                </button>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <button
                                    onclick={ctx.link().callback({
                                        let id = user_id.clone();
                                        move |_| Msg::SetEditMode(Some(id.clone()))
                                    })}
                                    style="padding: 4px 8px; margin-right: 5px; background-color: #ffc107; color: black; border: none; border-radius: 2px; cursor: pointer;"
                                >
                                    {"Edit"}
                                </button>
                                <button
                                    onclick={ctx.link().callback({
                                        let id = user_id.clone();
                                        move |_| Msg::DeleteUser(id.clone())
                                    })}
                                    style="padding: 4px 8px; background-color: #dc3545; color: white; border: none; border-radius: 2px; cursor: pointer;"
                                >
                                    {"Delete"}
                                </button>
                            </>
                        }
                    }}
                </td>
            </tr>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
