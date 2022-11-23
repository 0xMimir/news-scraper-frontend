use yew::{UseStateHandle, use_context};
use yew_router::prelude::{AnyHistory, use_history, History};
use std::fmt;

use crate::{routes::AppRoute, services::storage::{set_user, remove_user}};

use super::user::UserInfo;

#[derive(Clone)]
pub struct UseStoreContextHandle{
    user: UseStateHandle<UserInfo>,
    history: AnyHistory
}

impl fmt::Debug for UseStoreContextHandle{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.user))
            .finish()
    }
}

pub fn get_store() -> UseStoreContextHandle{
    let user = use_context().unwrap();
    let history = use_history().unwrap();

    UseStoreContextHandle{user, history}
}

impl UseStoreContextHandle{
    pub fn login(&self, user: UserInfo){
        set_user(user.clone());
        self.user.set(user);
        self.history.push(AppRoute::Home)
    }
    pub fn logout(&self){
        remove_user();
        self.user.set(UserInfo::default());
        self.history.push(AppRoute::Home)
    }
    pub fn get_user(&self) -> UseStateHandle<UserInfo>{
        self.user.clone()
    }
}
