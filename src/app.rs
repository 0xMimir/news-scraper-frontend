use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Navbar, StoreProvider};
use crate::routes::{switch, AppRoute};

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
