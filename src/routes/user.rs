use yew::{function_component, html, use_context, Html};

use crate::{store::{user::UserState, objects::user::Plans}, components::PlansComponent};
// use yew_hooks::use_clipboard;

// use crate::helpers::storage::get_key;
// use crate::context::get_store;
// use crate::store::objects::user::Plans;

// use super::AppRoute;
// use crate::components::PlansComponent;

#[function_component(User)]
pub fn user() -> Html {
    let user = use_context::<UserState>().unwrap();

  
    html! {
        <>
            <br />
            <h2>{&user.username}</h2>
            <div class="row">
                <div class="col-md">
                    <span>{format!("Email: {}", &user.email)}</span> <br />
                    <span>{format!("Plan: {:?}", &user.plan)}</span> <br />
                    <p>{"Api key:"}</p>
                    <code style="background-color: rgba(0,0,0, 0.4);padding: 8px">
                                {&user.api_key}
                                //<button {onclick} style="background: none;margin-left: 5px;border: none;color: white;margin-right: 0;">
                                //    <i class="bi bi-clipboard"></i>
                                //</button>
                    </code>
                </div>
                <div class="col-md">
                    <span>{format!("Calls used: {}", &user.credits_used)}</span> <br />
                    <span>{format!("Calls remaining: {}", &user.credits_remaining)}</span> <br />
                    <span>{format!("Max calls: {}", &user.plan.get_max_calls())}</span> <br />
                    //<span>{format!("Restarts in: ")}</span>
                </div>
            </div>
            <br />
            <br />
            {
                match user.plan{
                    Plans::Free => html!{<PlansComponent />},
                    _ => html!{}
                }
            }
        </>
    }
}
