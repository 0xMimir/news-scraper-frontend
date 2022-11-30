use yew::{function_component, html};
use yew_router::prelude::{use_history, History};

use crate::store::{get_store, Plans};
use crate::routes::{ScrapersInfo};

use super::AppRoute;

#[function_component(AdminPage)]
pub fn admin_page() -> Html{
    let user = get_store().get_user();

    if user.plan != Plans::Staff{
        let history = use_history().unwrap();
        history.push(AppRoute::PageNotFound);
        return html! {};
    }

    html!{
        <>
            <h1>{"Admin Dashboard"}</h1>
            <ScrapersInfo />
        </>
    }
}