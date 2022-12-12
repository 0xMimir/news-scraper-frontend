use yew::{function_component, html, Html};
// use yew_hooks::use_clipboard;
// use yew_router::prelude::{use_history, History};

// use crate::helpers::storage::get_key;
// use crate::context::get_store;
// use crate::store::objects::user::Plans;

// use super::AppRoute;
// use crate::components::PlansComponent;

#[function_component(User)]
pub fn user() -> Html {
    // if get_key().is_none() {
    //     let history = use_history().unwrap();
    //     history.push(AppRoute::PageNotFound);
    //     return html! {};
    // }

    // let user = get_store().get_user();

    // let clipboard = use_clipboard();
    // let onclick = {
    //     let api_key = user.api_key.clone();
    //     Callback::from(move |_| clipboard.write_text(api_key.clone()))
    // };

    // html! {
    //     <>
    //         <br />
    //         <h2>{&user.username}</h2>
    //         <div class="row">
    //             <div class="col-md">
    //                 <span>{format!("Email: {}", &user.email)}</span> <br />
    //                 <span>{format!("Plan: {:?}", &user.plan)}</span> <br />
    //                 <p>{"Api key:"}</p>
    //                 <code style="background-color: rgba(0,0,0, 0.4);padding: 8px">
    //                             {&user.api_key}
    //                             <button {onclick} style="background: none;margin-left: 5px;border: none;color: white;margin-right: 0;">
    //                                 <i class="bi bi-clipboard"></i>
    //                             </button>
    //                 </code>
    //             </div>
    //             <div class="col-md">
    //                 <span>{format!("Calls used: {}", &user.credits_used)}</span> <br />
    //                 <span>{format!("Calls remaining: {}", &user.credits_remaining)}</span> <br />
    //                 <span>{format!("Max calls: {}", &user.plan.get_max_calls())}</span> <br />
    //                 //<span>{format!("Restarts in: ")}</span>
    //             </div>
    //         </div>
    //         <br />
    //         <br />
    //         {
    //             #[allow(clippy::let_unit_value)] // Unknow error in yew
    //             match user.plan{
    //                 Plans::Free => html!{<PlansComponent />},
    //                 _ => html!{}
    //             }
    //         }
    //     </>
    // }
    html!{
        "user"
    }
}
