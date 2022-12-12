use yew::{function_component, html, Html, use_context};

use crate::components::{AdminModule, Forbiden, ScrapersInfo, ShowUsers};
use crate::store::objects::user::Plans;
use crate::store::user::UserState;

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    let user = use_context::<UserState>().unwrap();
    let scrapers = html! { <ScrapersInfo />};
    let users = html! { <ShowUsers />};

    match user.plan {
        Plans::Staff => html! {
            <>
                <h1>{"Admin Dashboard"}</h1>
                <AdminModule title="Scrapers info" module={scrapers} />
                <AdminModule title="Users" module={users} />
            </>
        },
        _ => {
            html! {
                <Forbiden />
            }
        }
    }
}
