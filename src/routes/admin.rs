use yew::{function_component, html, Html};

// use crate::components::{AdminModule, Forbiden, ShowUsers};
// use crate::{context::get_store, routes::ScrapersInfo, store::objects::user::Plans};

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    // let user = get_store().get_user();
    // let scrapers = html! { <ScrapersInfo />};
    // let users = html! { <ShowUsers />};

    // match user.plan {
    //     Plans::Staff => html! {
    //         <>
    //             <h1>{"Admin Dashboard"}</h1>
    //             <AdminModule title="Scrapers info" module={scrapers} />
    //             <AdminModule title="Users" module={users} />
    //         </>
    //     },
    //     _ => {
    //         html! {
    //             <Forbiden />
    //         }
    //     }
    // }
    html!{
        "admin"
    }
}
