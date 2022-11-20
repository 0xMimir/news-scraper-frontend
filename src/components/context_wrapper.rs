use yew::{Children, function_component, use_state, Properties, html, ContextProvider, UseStateHandle, use_effect_with_deps};
use yew_hooks::{use_async, use_mount};

use crate::{store::UserInfo, services::{AuthStore, storage::get_key}};

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub children: Children
}

#[function_component(StoreProvider)]
pub fn store_provider(props: &Props) -> Html{
    let store = use_state(UserInfo::default);
    let update_user = use_async(async move {AuthStore::current().await});

    {
        let update_user = update_user.clone();
        use_mount(move || {
            if get_key().is_some(){
                update_user.run();
            }
        })
    }
    
    {
        let store = store.clone();
        use_effect_with_deps(
            move | update_user | {
                if let Some(user) = &update_user.data{
                    store.set(user.clone());
                } || ()
            }, 
            update_user
        )
    }
    
    html!{
        <ContextProvider<UseStateHandle<UserInfo>> context={store}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}