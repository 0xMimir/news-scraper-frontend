use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
mod user;

use about::About;
use home::Home;
use user::User;

use crate::components::Plans;

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
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Plans => html! { <Plans /> },
        AppRoute::User => html!{ <User /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
