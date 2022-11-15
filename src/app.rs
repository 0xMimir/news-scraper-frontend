use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Nav, StoreProvider};
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <StoreProvider>
            <BrowserRouter>
                <Nav />
                <div class="container">
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        </StoreProvider>
    }
}
