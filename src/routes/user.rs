use yew::{function_component, html, Callback};
use yew_hooks::use_clipboard;
use yew_router::prelude::{use_history, History};

use crate::{store::get_store, services::storage::get_key};

use super::AppRoute;
use crate::components::Plans;

#[function_component(User)]
pub fn user() -> Html{
    if get_key().is_none(){
        let history = use_history().unwrap();
        history.push(AppRoute::PageNotFound);
        return html!{};
    }

    let user = get_store().get_user();
    
    let clipboard = use_clipboard();
    let onclick = {
        let clipboard = clipboard.clone();
        let api_key = user.api_key.clone();
        Callback::from(move |_| {
            clipboard.write_text(api_key.clone())
        })
    };

    html!{
        <>
            <div class="row">
                <div class="col-1">
                    <i class="bi bi-person-circle" style="font-size: 80px"></i>
                </div>
                <div class="col-11">
                    <br />
                    <h2>{&user.username}</h2>
                    <span>{format!("Email: {}", &user.email)}</span> <br />
                    <span>{format!("Plan: {}", &user.plan)}</span> <br />
                    <p>{"Api key:"}</p>
                    <code style="background-color: rgba(0,0,0, 0.4);padding: 8px">
                        {&user.api_key}
                        <button {onclick} style="background: none;margin-left: 5px;border: none;color: white;margin-right: 0;">
                            <i class="bi bi-clipboard"></i>
                        </button>
                    </code>
                </div>
            </div>
            <br />
            <br />
            {
                if user.plan == "Free"{
                    html!{<Plans />}
                }else{
                    html!{}
                }
            }
        </>
    }
}