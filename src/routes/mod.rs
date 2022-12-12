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

use crate::components::{PlansComponent, NotFound, Container};


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

    html! {
        <Container>
            {match route{
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
