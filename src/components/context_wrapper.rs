use yew::{Children, function_component, use_state, Properties, html, ContextProvider, UseStateHandle};

use crate::{store::UserInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub children: Children
}

#[function_component(StoreProvider)]
pub fn store_provider(props: &Props) -> Html{
    let store = use_state(UserInfo::default);
    
    html!{
        <ContextProvider<UseStateHandle<UserInfo>> context={store}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}