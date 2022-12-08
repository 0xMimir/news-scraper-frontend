use yew::{function_component, html};
use crate::{components::Plan, store::objects::user::Plans};

#[function_component(PlansComponent)]
pub fn plans_component() -> Html{
    html!{
        <div class="align-middle align-self-center text-center">
            <h1>{"Plans"}</h1>
            <p>{"Available plans"}</p>
            <div class="row">
                <Plan
                    plan_name={Plans::Free}
                    calls={(100, "day".to_owned())}
                    functionality={vec![
                        "Latest news only".to_owned(),
                        "Partial article data".to_owned()
                    ]}
                    price={0.}
                />
                <Plan
                    plan_name={Plans::Basic}
                    calls={(20_000, "month".to_owned())}
                    functionality={vec![
                        "Search news for keywords".to_owned(),
                        "Sentiment analysis of articles".to_owned(),
                        "Full article data".to_owned()
                    ]}
                    price={9.99}
                    color={"#262627"}
                    button_color={"#5161FD"}
                    border_color={"#5161FD"}
                    button_text_color={"#FEFDFE"}
                />
                <Plan
                    plan_name={Plans::Premium}
                    calls={(50_000, "month".to_owned())}
                    functionality={vec![
                        "Full functionality from previus plans".to_owned(),
                        "Additonal routes".to_owned(),
                        "Analysis of sources".to_owned(),
                        "Sentiment analysis".to_owned(),
                    ]}
                    price={49.99}
                    color={"#5160FD"}
                    button_color={"#FEFDFE"}
                    border_color={"#FEFDF3"}
                    button_text_color={"#262627"}
                />
            </div>
        </div>
    }
}