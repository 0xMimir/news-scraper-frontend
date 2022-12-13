use yew::{
    function_component, html, use_state, Children, Component, Context,
    ContextProvider as CTXProvider, Html, Properties,
};

use crate::{
    helpers::storage::{get_key, set_user},
    store::{
        objects::user::User,
        user::{UserState, UserStore},
    },
    Error,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[derive(Properties, Clone, PartialEq)]
struct InnerContextProps {
    pub children: Children,
    pub user: User,
}

pub struct ContextProvider {
    state: User,
}

impl Component for ContextProvider {
    type Message = Result<User, Error>;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let state = User::default();
        if get_key().is_some() {
            ctx.link()
                .send_future(async move { UserStore::current().await });
        }

        Self { state }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = match msg {
            Ok(user) => user,
            Err(_) => {
                UserStore::logout();
                User::empty()
            },
        };
        set_user(self.state.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ContextProviderInner user={self.state.clone()}>
                {for ctx.props().children.clone()}
            </ContextProviderInner>
        }
    }
}

#[function_component(ContextProviderInner)]
fn context_provider_inner(props: &InnerContextProps) -> Html {
    let state = use_state(|| props.user.clone());

    html! {
        <CTXProvider<UserState> context={state}>
            {for props.children.iter()}
        </CTXProvider<UserState>>
    }
}
