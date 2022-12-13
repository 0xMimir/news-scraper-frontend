use yew::{html, Html};

mod about;
mod admin;
mod home;
mod user;

use about::About;
use admin::AdminPage;
use home::Home;
use user::User;
use yew_router::Routable;

use crate::{
    components::{Container, NotFound, PlansComponent},
    helpers::storage::get_key,
};

/// App routes
#[derive(Routable, PartialEq, Eq, Clone, Debug)]
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

pub fn switch(route: AppRoute) -> Html {
    if route == AppRoute::PageNotFound {
        return html! { <NotFound /> };
    }

    let valid_route = match (route, get_key().is_some()) {
        (AppRoute::Home, _) => Some(html! { <Home /> }),
        (AppRoute::About, _) => Some(html! { <About /> }),
        (AppRoute::Plans, _) => Some(html! { <PlansComponent /> }),
        (AppRoute::User, true) => Some(html! { <User /> }),
        (AppRoute::AdminPage, true) => Some(html! { <AdminPage />}),
        // This should never happen
        _ => None,
    };

    match valid_route {
        Some(html) => html! {
            <Container>
                {html}
            </Container>
        },
        None => html! { <NotFound /> },
    }
}
