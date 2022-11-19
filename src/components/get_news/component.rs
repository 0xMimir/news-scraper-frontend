use yew::{function_component, html};
use yew_hooks::{use_async, use_is_first_mount};
use thousands::Separable;

use crate::services::NewsStore;
use crate::components::ShowNews;

#[function_component(GetNews)]
pub fn get_news() -> Html{
    let state = use_async(async move {
        NewsStore::get_news().await
    });

    if use_is_first_mount(){
        state.run();
    }

    let (news, news_count) = if let Some(response) = &state.data{
        let news_count = (response.total / 1000) * 1000;
        (response.items.clone(), news_count)
    }else{
        (vec![], 0)
    };

    html!{
        <div class="text-center" style="padding-top: 7vh">
            <h3>{"Get latest news"}</h3>
            <p>{format!("Over {}+ news entires", news_count.separate_with_commas())}</p>
            <ShowNews news={news.clone()} />
        </div>
    }
}