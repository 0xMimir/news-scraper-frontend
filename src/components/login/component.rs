use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{InputEvent, Event, HtmlInputElement, FocusEvent};
use yew::{function_component, html, Callback, use_state, use_effect_with_deps};
use yew_hooks::use_async;

use crate::store::{UserInfo, get_store};

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
        use_async(async move {
            UserInfo::login(&username, &password).await
        })
    };

    let username_callback = Callback::from(move |input: InputEvent| {
        let event: Event = input.dyn_into().unwrap_throw();
        let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        username_handle.set(input_elem.value())
    });

    let password_callback = Callback::from(move |input: InputEvent| {
        let event: Event = input.dyn_into().unwrap_throw();
        let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        password_handle.set(input_elem.value())
    });

    let login_callback = {
        let handle = login_handle.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            handle.run();
        }) 
    };

    use_effect_with_deps(
        move |login_handle| {
            if let Some(user) = &login_handle.data{
                store.login(user.clone());
            }
            || ()
        },
        login_handle.clone()
    );

    html!{
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