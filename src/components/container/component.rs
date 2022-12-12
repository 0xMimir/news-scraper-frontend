use yew::{Children, function_component, Properties, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub children: Children
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html{
    html!{
        <div class="container">
            { for props.children.iter() }
        </div>
    }
}