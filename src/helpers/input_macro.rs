#[macro_export]
macro_rules! input_callback {
    ($name:ident) => {
        Callback::from(move |input: InputEvent| {
            let event: Event = input.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            $name.set(input_elem.value())
        })
    };
}