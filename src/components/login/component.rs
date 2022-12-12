use crate::helpers::storage::set_user;
use crate::{input_callback, Error};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent, SubmitEvent};
use yew::platform::spawn_local;
use yew::{function_component, html, use_context, use_state, Callback, Html};

use crate::store::user::{UserState, UserStore};

use crate::components::ShowError;

#[allow(clippy::let_unit_value)]
#[function_component(Login)]
pub fn login() -> Html {
    let store = use_context::<UserState>().unwrap();

    let username_handle = use_state(|| "".to_string());
    let password_handle = use_state(|| "".to_string());
    let error_state = use_state(|| None::<Error>);

    let login_callback = {
        let username_handle = username_handle.clone();
        let password_handle = password_handle.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username_handle).clone();
            let password = (*password_handle).clone();
            let store = store.clone();
            let error_state = error_state.clone();
            spawn_local(async move {
                match UserStore::login(&username, &password).await {
                    Ok(user) => {
                        store.set(user.clone());
                        set_user(user);
                    }
                    Err(error) => error_state.set(Some(error)),
                };
            });
        })
    };

    let username_callback = input_callback!(username_handle);
    let password_callback = input_callback!(password_handle);

    html! {
        <div class="inner-form">
            <h1>{"Login"}</h1>
            <form onsubmit={login_callback}>
                <ShowError error={(*error_state).clone()} />
                <div class="form-outline mb-4">
                    <label class="form-label">{"Username"}</label>
                    <input type="text" oninput={username_callback} class="form-control" />
                </div>
                <div class="form-outline mb-4">
                    <label class="form-label">{"Password"}</label>
                    <input type="password" oninput={password_callback} class="form-control" />
                </div>
                <br />
                <button type="submit" class="btn btn-primary btn-block mb-4" >{"Sign in"}</button>
            </form>
        </div>
    }
}
