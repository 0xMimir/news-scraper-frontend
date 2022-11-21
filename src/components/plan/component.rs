use thousands::Separable;
use yew::{function_component, html, Html, Properties};

use crate::store::Plans;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plan_name: Plans,
    pub calls: (i32, String),
    pub functionality: Vec<String>,
    pub price: f64,
    pub color: Option<String>,
    pub border_color: Option<String>,
    pub button_color: Option<String>,
    pub button_text_color: Option<String>
}

#[function_component(Plan)]
pub fn plan(props: &Props) -> Html {
    let (calls, time) = &props.calls;
    let mut button_css = String::new();

    let background_color = match &props.color{
        Some(color) => color,
        None => "#1C1C1D"
    };

    button_css.push_str("border: 2px solid ");
    button_css.push_str(match &props.border_color{
        Some(color) => color,
        None => "#252524"
    });
    button_css.push(';');

    button_css.push_str("background-color: ");
    button_css.push_str(match &props.button_color{
        Some(color) => color,
        None => "inherit"
    });
    button_css.push(';');

    button_css.push_str("color: ");
    button_css.push_str(match &props.button_text_color{
        Some(color) => color,
        None => "#D9D9D8"
    });
    button_css.push(';');


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
                <button style={button_css}>
                    {"Subscribe now"}
                </button>
            </div>
        </div>
    }
}
