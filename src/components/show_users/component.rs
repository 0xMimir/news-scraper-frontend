use yew::{function_component, html};
use yew_hooks::{use_async, use_is_first_mount, use_interval};

use crate::store::user::UserStore;

#[function_component(ShowUsers)]
pub fn show_users() -> Html {
    let users = use_async(async move {UserStore::get_users().await});
    
    if use_is_first_mount() {
        users.run();
    }

    {
        let state = users.clone();
        use_interval(move || state.run(), 2000)
    }

    html!{
        {if let Some(users_) = &users.data{
            html!{
                format!("{:?}", users_.items[0])
            }
        }else{
            html!{}
        }}
    }
}