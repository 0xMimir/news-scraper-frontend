use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Navbar;
use crate::routes::{switch, AppRoute};
use crate::context::ContextProvider;

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ContextProvider>
            <BrowserRouter>
                <Navbar />
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider>
    }
}
