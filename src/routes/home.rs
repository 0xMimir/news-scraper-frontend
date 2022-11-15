use yew::{function_component, html};
use yew_hooks::{use_async, use_is_first_mount};

use crate::{components::{GetNews, SearchNews}, services::NewsStore};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let sources_count = use_async(async move {
        NewsStore::get_news_count().await
    });

    if use_is_first_mount(){
        sources_count.run()
    }

    let sources_count = {
        match &sources_count.data{
            Some(sources) => {
                (sources / 5) * 5
            },
            None => 20
        }
    };
    
    html!{
        <div class="align-middle align-self-center text-center">
            <h1>{"Bespoke Crypto News API"}</h1>
            <p>
                { "Get news from over " }
                { sources_count }
                { "+ sources" }
            </p>
            <div class="text-center" style="padding-top: 7vh">
                <h3>{"As simple as it gets"}</h3>
                <p>{"Two routes for everyhing you need"}</p>
                <div class="row">
                    <div class="col-6">
                        <code>{"/news"}</code>
                        <p>
                            {"Get news, with 5 params to filter for your needs"}
                        </p>
                        <ul>
                            <li>{"Older than"}</li>
                            <li>{"Newer than"}</li>
                            <li>{"Coin"}</li>
                            <li>{"Containing"}</li>
                            <li>{"Not containing"}</li>
                        </ul>
                    </div>
                    <div class="col-6">
                        <code>{"/news/search"}</code>
                        <p>
                            {"Search news, either based on title or content"}
                        </p>
                        <ul>
                            <li>{"Title"}</li>
                            <li>{"Content"}</li>
                        </ul>
                    </div>
                </div>
            </div>
            <GetNews />
            <SearchNews />
        </div>
    }
}
