use yew::{html, Html};

mod about;
mod home;
mod user;
mod admin;

use about::About;
use home::Home;
use user::User;
use admin::AdminPage;
use yew_router::Routable;

use crate::components::*;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/plans")]
    Plans,
    #[at("/self")]
    User,
    #[at("/admin")]
    AdminPage,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
#[allow(clippy::let_unit_value)]
pub fn switch(routes: &AppRoute) -> Html {
    if routes == &AppRoute::PageNotFound{
        return html!{ <NotFound /> };
    }
    
    html!{
        <Container>
            {match routes{
                AppRoute::Home => html! { <Home /> },
                AppRoute::About => html! { <About /> },
                AppRoute::Plans => html! { <PlansComponent /> },
                AppRoute::User => html! { <User /> },
                AppRoute::AdminPage => html! { <AdminPage />},
                // This should never happen
                AppRoute::PageNotFound => html!{}
            }}
        </Container>
    }
}
