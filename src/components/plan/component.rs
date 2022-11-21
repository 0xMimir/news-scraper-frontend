use thousands::Separable;
use yew::{function_component, html, Html, Properties};

use crate::store::Plans;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plan_name: Plans,
    pub calls: (i32, String),
    pub functionality: Vec<String>,
    pub price: f64,
    pub color: Option<String>
}

#[function_component(Plan)]
pub fn plan(props: &Props) -> Html {
    let (calls, time) = &props.calls;

    let background_color = match &props.color{
        Some(color) => color,
        None => "#1C1C1D"
    };

    html! {
        <div class="col-md plan-div-border">
            <div class="plan-div" style={format!("background-color: {}", background_color)}>
                <h3>{if &props.plan_name == &Plans::Free{
                    {"Free".to_owned()}
                }else{
                    {format!("{:.2}$/mo", props.price)}
                }}</h3>
                <br />
                <ul>
                    <li>{format!("{} call per {}", calls.separate_with_commas(), time)}</li>
                    {props.functionality.iter().map(|func| html!{
                        <li>{func}</li>
                    }).collect::<Html>()}
                </ul>
                <button>
                    {"Subscribe now"}
                </button>
            </div>
        </div>
    }
}
