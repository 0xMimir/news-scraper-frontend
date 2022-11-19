use yew::{function_component, html};
use crate::components::Plan;

#[function_component(Plans)]
pub fn plans() -> Html{
    html!{
        <div class="align-middle align-self-center text-center">
            <h1>{"Plans"}</h1>
            <p>{"Available plans"}</p>
            <div class="row">
                <Plan
                    plan_name={"Free"}
                    calls={(100, "day".to_owned())}
                    functionality={vec![
                        "Latest news only".to_owned(),
                        "Partial article data".to_owned()
                    ]}
                    price={0.}
                />
                <Plan
                    plan_name={"Basic"}
                    calls={(20_000, "month".to_owned())}
                    functionality={vec![
                        "Search news for keywords".to_owned(),
                        "Sentiment analysis of articles".to_owned(),
                        "Full article data".to_owned()
                    ]}
                    price={9.99}
                />
                <Plan
                    plan_name={"Premium"}
                    calls={(50_000, "month".to_owned())}
                    functionality={vec![
                        "Full functionality from previus plans".to_owned(),
                        "Additonal routes".to_owned(),
                        "Analysis of sources".to_owned(),
                        "Sentiment analysis".to_owned(),
                    ]}
                    price={49.99}
                />
            </div>
        </div>
    }
}