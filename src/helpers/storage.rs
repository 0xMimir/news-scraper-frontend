use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::store::objects::user::User;


const STORAGE_KEY: &str = "user";

lazy_static!{
    pub static ref TOKEN: RwLock<Option<User>> = {
        if let Ok(token) = LocalStorage::get(STORAGE_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

pub fn set_user(user: User){
    LocalStorage::set(STORAGE_KEY, &user)
        .expect("failed to set api_key");

    let mut token_lock = TOKEN.write();
    *token_lock = Some(user);
}

pub fn get_key() -> Option<String>{
    let key = get_user()?.api_key;
    match key.is_empty(){
        false => Some(key),
        true => None
    }
}

pub fn get_user() -> Option<User>{
    let token_lock = TOKEN.read();
    token_lock.clone()
}

pub fn remove_user(){
    LocalStorage::delete(STORAGE_KEY);
    let mut token_lock = TOKEN.write();
    *token_lock = None;
}