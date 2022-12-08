use yew::{
    function_component, html, use_effect_with_deps, use_state, Children, ContextProvider,
    Properties, UseStateHandle,
};
use yew_hooks::{use_async, use_mount};

use crate::helpers::storage::{get_key, set_user};

use super::user::UserStore;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(StoreProvider)]
pub fn store_provider(props: &Props) -> Html {
    let store = use_state(UserStore::default);
    let update_user = use_async(async move { UserStore::current().await });

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
                    store.set(UserStore::empty())
                }
                || ()
            },
            update_user,
        )
    }

    html! {
        <ContextProvider<UseStateHandle<UserStore>> context={store}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserStore>>>
    }
}
