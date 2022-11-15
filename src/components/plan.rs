use thousands::Separable;
use yew::{function_component, html, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub plan_name: String,
    pub calls: (i32, String),
    pub functionality: Vec<String>,
    pub price: f64
}

#[function_component(Plan)]
pub fn plan(props: &Props) -> Html{
    let (calls, time) = &props.calls;

    html!{
        <div class="col-4">
            <h3>{&props.plan_name}</h3>
            <br />
            <ul>
                <li>{format!("{} call per {}", calls.separate_with_commas(), time)}</li>
                {props.functionality.iter().map(|func| html!{
                    <li>{func}</li>
                }).collect::<Html>()}
            </ul>
            <h5>{format!("{:.2}$/month", props.price)}</h5>
        </div>
    }
}