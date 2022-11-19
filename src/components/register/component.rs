use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{InputEvent, Event, HtmlInputElement, FocusEvent};
use yew::{function_component, html, Callback, use_state, use_effect_with_deps};
use yew_hooks::use_async;

use crate::store::{UserInfo, get_store};

use crate::components::ShowError;

#[function_component(Register)]
pub fn register() -> Html {
    let store = get_store();
    let username_handle = use_state(|| "".to_string());
    let password_handle = use_state(|| "".to_string());
    let email_handle = use_state(|| "".to_string());

    let register_handle = {
        let username = (*username_handle.clone()).clone();
        let password = (*password_handle.clone()).clone();
        let email = (*email_handle.clone()).clone();
        use_async(async move {
            UserInfo::register(&email, &username, &password).await
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

    let email_callback = Callback::from(move |input: InputEvent| {
        let event: Event = input.dyn_into().unwrap_throw();
        let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        email_handle.set(input_elem.value())
    });

    let register_callback = {
        let handle = register_handle.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            handle.run();
        }) 
    };

    use_effect_with_deps(
        move |register_handle| {
            if let Some(user) = &register_handle.data{
                store.login(user.clone());
            }
            || ()
        },
        register_handle.clone()
    );

    html!{
        <div class="inner-form">
            <h1>{"Login"}</h1>
            <form onsubmit={register_callback }>
                <ShowError error={register_handle.error.clone()} />
                <div class="form-outline mb-4">
                    <label class="form-label">{"Email"}</label>
                    <input type="email" oninput={email_callback} class="form-control" />
                </div>
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