use crate::input_callback;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback};
use yew_hooks::use_async;

use crate::context::get_store;
use crate::store::user::UserStore;

use crate::components::ShowError;

#[allow(clippy::let_unit_value)]
#[function_component(Login)]
pub fn login() -> Html {
    let store = get_store();
    let username_handle = use_state(|| "".to_string());
    let password_handle = use_state(|| "".to_string());

    let login_handle = {
        let username = (*username_handle).clone();
        let password = (*password_handle).clone();
        use_async(async move { UserStore::login(&username, &password).await })
    };

    let username_callback = input_callback!(username_handle);
    let password_callback = input_callback!(password_handle);

    let login_callback = {
        let handle = login_handle.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            handle.run();
        })
    };

    use_effect_with_deps(
        move |login_handle| {
            if let Some(user) = &login_handle.data {
                store.login(user.clone());
            }
            || ()
        },
        login_handle.clone(),
    );

    html! {
        <div class="inner-form">
            <h1>{"Login"}</h1>
            <form onsubmit={login_callback}>
                <ShowError error={login_handle.error.clone()} />
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
