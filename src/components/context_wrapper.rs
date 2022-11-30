use yew::{
    function_component, html, use_effect_with_deps, use_state, Children, ContextProvider,
    Properties, UseStateHandle,
};
use yew_hooks::{use_async, use_mount};

use crate::{
    services::{storage::{get_key, set_user}, AuthStore},
    store::UserInfo,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(StoreProvider)]
pub fn store_provider(props: &Props) -> Html {
    let store = use_state(UserInfo::default);
    let update_user = use_async(async move { AuthStore::current().await });

    {
        let update_user = update_user.clone();
        use_mount(move || {
            if get_key().is_some() {
                update_user.run();
            }
        })
    }

    {
        let store = store.clone();
        use_effect_with_deps(
            move |update_user| {
                if let Some(user) = &update_user.data {
                    set_user(user.clone());
                    store.set(user.clone());
                }
                if update_user.error.is_some() {
                    store.set(UserInfo::empty())
                }
                || ()
            },
            update_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={store}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
