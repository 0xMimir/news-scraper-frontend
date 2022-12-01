use yew::{function_component, html, Properties, Html};

use crate::services::NewsEntry;

#[derive(Properties, PartialEq, Eq)]
pub struct Props{
    pub news: Vec<NewsEntry>
}

#[function_component(ShowNews)]
pub fn show_news(props: &Props) -> Html{
    html!{
        <div class="show-news-container">{
            {
                props.news
                    .iter()
                    .map(|e| e.to_html())
                    .collect::<Html>()
            }
        }</div>
    }
}