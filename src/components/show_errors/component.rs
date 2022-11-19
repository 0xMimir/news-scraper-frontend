use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component(ShowError)]
pub fn show_errors(props: &Props) -> Html {
    if let Some(error) = &props.error {
        html! {
            <div class="error-message">
                <h5>{error.to_string()}</h5>
            </div>
        }
    } else {
        html! {}
    }
}
