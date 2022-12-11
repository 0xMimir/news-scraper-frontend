use web_sys::MouseEvent;
use yew::{UseStateHandle, Callback};

#[macro_export]
macro_rules! input_callback {
    ($name:ident) => {
        Callback::from(move |input: InputEvent| {
            let event: Event = input.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement =
                event.target().unwrap_throw().dyn_into().unwrap_throw();
            $name.set(input_elem.value())
        })
    };
}

pub fn button_switch(state: &UseStateHandle<bool>) -> Callback<MouseEvent>{
    let state = state.clone();
    Callback::from(move |_| state.set(!*state))
}