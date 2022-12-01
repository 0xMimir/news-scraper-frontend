use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Login, Register};
use crate::services::helpers::API_ROOT;
use crate::store::Plans;
use crate::{routes::AppRoute, services::storage::get_key, store::get_store};

/// Nav component
#[function_component(Navbar)]
pub fn nav() -> Html {
    let store = get_store();
    let loged_in = get_key().is_some();
    let register = use_state_eq(|| false);
    let state = use_state_eq(|| false);
    let user = &store.get_user();

    let open_modal = {
        let state = state.clone();
        Callback::from(move |_| state.set(true))
    };

    let close_modal = {
        let state = state.clone();
        Callback::from(move |_| state.set(false))
    };

    let logout = { Callback::from(move |_: MouseEvent| store.logout()) };

    let register_callback = {
        let register = register.clone();
        Callback::from(move |_| register.set(!*register))
    };

    html! {
        <nav class="navbar navbar-expand-lg">
            <span class="navbar-brand"><Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ " Bespoke News " }</Link<AppRoute>></span>
            <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarTogglerDemo02">
                <i class="bi bi-arrow-down-circle app-link"></i>
            </button>
            <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
                <ul class="navbar-nav mr-auto mt-2 mt-lg-0">
                    <li class="nav-item li-space">
                        <Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ " Home " }</Link<AppRoute>>
                    </li>
                    <li class="nav-item li-space">
                        <Link<AppRoute> to={AppRoute::Plans} classes="app-link" >{ " Plans " }</Link<AppRoute>>
                    </li>
                    <li class="nav-item li-space">
                        <Link<AppRoute> to={AppRoute::About} classes="app-link" >{ " About " }</Link<AppRoute>>
                    </li>
                    {
                        if user.plan == Plans::Staff{
                            html!{
                                <li class="nav-item li-space">
                                    <Link<AppRoute> to={AppRoute::AdminPage} classes="app-link" >{ " Dashboard " }</Link<AppRoute>>
                                </li>
                            }
                        }else{
                            html!{}
                        }
                    }
                    <li class="nav-item li-space">
                        <a href={format!("{}/swagger-ui/#/", API_ROOT)} class="app-link">{" Docs "}</a>
                    </li>
                </ul>
                <ul class="nav navbar-nav navbar-right">
                    <li>
                        {if loged_in{
                            html!{
                                <div class="dropdown">
                                    <button
                                        class="btn dropdown-toggle"
                                        type="button"
                                        id="dropdownMenuButton"
                                        data-toggle="dropdown"
                                        style="color:white"
                                    >
                                        <Link<AppRoute> to={AppRoute::User} classes="app-link" >{ &user.username }</Link<AppRoute>>
                                    </button>
                                    <div
                                        class="dropdown-menu"
                                        aria-labelledby="dropdownMenuButton"
                                        style="background-color: rgba(0,0,0,0);border:none"
                                    >
                                        <button
                                            onclick={logout}
                                            style="color:#61D9FB;border:none;background:none"
                                        >
                                            {"Logout"}
                                        </button>
                                    </div>
                                </div>
                            }
                        }else{
                            html!{
                                <button onclick={open_modal} class="btn" style="color:#61D9FB">
                                    {"Login/Register"}
                                </button>
                            }
                        }}
                    </li>
                </ul>
            </div>
            {
                if *state && !loged_in{
                    html!{
                        <div class="modul-wrapper">
                            <div class="modul">
                                <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                                    <button onclick={close_modal} class="btn btn-link">
                                        {"X"}
                                    </button>
                                </div>
                                <div>
                                    {
                                        #[allow(clippy::let_unit_value)] // Unknow error in yew
                                        if *register{
                                            html!{<Register />}
                                        }else{
                                            html!{<Login />}
                                        }
                                    }
                                </div>
                                <div style="text-align: center">
                                    <button onclick={register_callback} style="btn">{
                                        if *register{
                                            "Already a user, Login"
                                        }else{
                                            "Not a user, Register"
                                        }
                                    }</button>
                                </div>
                            </div>
                        </div>
                    }
                }else{
                    html!{<></>}
                }
            }
        </nav>
    }
}
