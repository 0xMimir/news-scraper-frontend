use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Navbar;
use crate::routes::{switch, AppRoute};
use crate::store::context_provider::StoreProvider;

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <StoreProvider>
            <BrowserRouter>
                <Navbar />
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </StoreProvider>
    }
}
