use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::components::Navbar;
use crate::context::ContextProvider;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ContextProvider>
            <BrowserRouter>
                <Navbar />
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </ContextProvider>
    }
}
