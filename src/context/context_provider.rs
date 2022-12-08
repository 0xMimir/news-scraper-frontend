use yew::{
    function_component, html, use_effect_with_deps, use_state, Children, ContextProvider as CTXProvider,
    Properties, UseStateHandle,
};
use yew_hooks::{use_async, use_mount};

use crate::{helpers::storage::{get_key, set_user}, store::{objects::user::User, user::UserStore}};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(ContextProvider)]
pub fn context_provider(props: &Props) -> Html {
    let store = use_state(User::default);
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
                    store.set(User::default())
                }
                || ()
            },
            update_user,
        )
    }

    html! {
        <CTXProvider<UseStateHandle<User>> context={store}>
            { for props.children.iter() }
        </CTXProvider<UseStateHandle<User>>>
    }
}
