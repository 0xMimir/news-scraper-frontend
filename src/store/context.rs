use yew::{use_context, UseStateHandle};
use yew_router::prelude::{use_history, AnyHistory, History};

use crate::{
    helpers::storage::{remove_user, set_user},
    routes::AppRoute,
};

use super::user::UserStore;

#[derive(Clone)]
pub struct UseStoreContextHandle {
    user: UseStateHandle<UserStore>,
    history: AnyHistory,
}

pub fn get_store() -> UseStoreContextHandle {
    let user = use_context().unwrap();
    let history = use_history().unwrap();
    UseStoreContextHandle { user, history }
}

impl UseStoreContextHandle {
    pub fn login(&self, user: UserStore) {
        set_user(user.clone());
        self.user.set(user);
        self.history.push(AppRoute::Home)
    }
    pub fn logout(&self) {
        remove_user();
        self.user.set(UserStore::default());
        self.history.push(AppRoute::Home)
    }
    pub fn get_user(&self) -> UseStateHandle<UserStore> {
        self.user.clone()
    }
}
