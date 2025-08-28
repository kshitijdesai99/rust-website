use crate::{models::{CreateUserRequest, UpdateUserRequest, User}, services::UserApi};
use yew::prelude::*;

/// User management state and operations
#[derive(Debug, Clone, PartialEq)]
pub struct UsersState {
    pub users: Vec<User>,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for UsersState {
    fn default() -> Self {
        Self {
            users: Vec::new(),
            loading: false,
            error: None,
        }
    }
}

/// Actions for user management
#[derive(Debug)]
pub enum UsersAction {
    SetLoading(bool),
    SetUsers(Vec<User>),
    AddUser(User),
    UpdateUser(User),
    RemoveUser(String),
    SetError(Option<String>),
}

impl Reducible for UsersState {
    type Action = UsersAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            UsersAction::SetLoading(loading) => {
                new_state.loading = loading;
            }
            UsersAction::SetUsers(users) => {
                new_state.users = users;
                new_state.loading = false;
                new_state.error = None;
            }
            UsersAction::AddUser(user) => {
                new_state.users.push(user);
                new_state.error = None;
            }
            UsersAction::UpdateUser(updated_user) => {
                if let Some(index) = new_state.users.iter().position(|u| u.id == updated_user.id) {
                    new_state.users[index] = updated_user;
                }
                new_state.error = None;
            }
            UsersAction::RemoveUser(id) => {
                new_state.users.retain(|u| u.id != id);
                new_state.error = None;
            }
            UsersAction::SetError(error) => {
                new_state.error = error;
                new_state.loading = false;
            }
        }

        std::rc::Rc::new(new_state)
    }
}

/// Custom hook for user management
#[hook]
pub fn use_users() -> (UseReducerHandle<UsersState>, UsersActions) {
    let state = use_reducer(UsersState::default);
    let actions = UsersActions::new(state.clone());
    
    (state, actions)
}

/// Actions interface for user management
#[derive(Clone, PartialEq)]
pub struct UsersActions {
    state: UseReducerHandle<UsersState>,
}

impl UsersActions {
    pub fn new(state: UseReducerHandle<UsersState>) -> Self {
        Self { state }
    }

    /// Load all users
    pub fn load_users(&self) {
        let state = self.state.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            state.dispatch(UsersAction::SetLoading(true));
            
            match UserApi::get_users().await {
                Ok(users) => {
                    state.dispatch(UsersAction::SetUsers(users));
                }
                Err(error) => {
                    state.dispatch(UsersAction::SetError(Some(error)));
                }
            }
        });
    }

    /// Create a new user
    pub fn create_user(&self, request: CreateUserRequest) {
        let state = self.state.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            state.dispatch(UsersAction::SetLoading(true));
            
            match UserApi::create_user(request).await {
                Ok(user) => {
                    state.dispatch(UsersAction::AddUser(user));
                    state.dispatch(UsersAction::SetLoading(false));
                }
                Err(error) => {
                    state.dispatch(UsersAction::SetError(Some(error)));
                }
            }
        });
    }

    /// Update an existing user
    pub fn update_user(&self, id: String, request: UpdateUserRequest) {
        let state = self.state.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            state.dispatch(UsersAction::SetLoading(true));
            
            match UserApi::update_user(&id, request).await {
                Ok(user) => {
                    state.dispatch(UsersAction::UpdateUser(user));
                    state.dispatch(UsersAction::SetLoading(false));
                }
                Err(error) => {
                    state.dispatch(UsersAction::SetError(Some(error)));
                }
            }
        });
    }

    /// Delete a user
    pub fn delete_user(&self, id: String) {
        let state = self.state.clone();
        let user_id = id.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            match UserApi::delete_user(&id).await {
                Ok(()) => {
                    state.dispatch(UsersAction::RemoveUser(user_id));
                }
                Err(error) => {
                    state.dispatch(UsersAction::SetError(Some(error)));
                }
            }
        });
    }

    /// Clear error
    pub fn clear_error(&self) {
        self.state.dispatch(UsersAction::SetError(None));
    }
}
