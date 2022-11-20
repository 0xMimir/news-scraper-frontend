use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
mod user;

use about::About;
use home::Home;
use user::User;

use crate::components::*;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/plans")]
    Plans,
    #[at("/self")]
    User,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    if routes == &AppRoute::PageNotFound{
        return html!{ <NotFound /> };
    }

    let html = match routes{
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Plans => html! { <PlansComponent /> },
        AppRoute::User => html!{ <User /> },
        AppRoute::PageNotFound => html!{}
    };

    html!{
        <Container>
            {html}
        </Container>
    }
}
