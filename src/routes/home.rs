use crate::{components::GetNews, store::news::NewsStore};
use yew::{function_component, html, platform::spawn_local, use_state, Html};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let sources_count = use_state(|| None);

    {
        let sources_count = sources_count.clone();

        if sources_count.is_none(){
            spawn_local(async move {
                let c = match NewsStore::get_news_count().await {
                    Ok(count) => (count / 5) * 5,
                    Err(_) => 20,
                };
                sources_count.set(Some(c));
            });
        }
    }

    html! {
        <div class="align-middle align-self-center text-center">
            <h1>{"Bespoke Crypto News API"}</h1>
            <p>
                { "Get news from over " }
                { *sources_count }
                { "+ sources" }
            </p>
            <div class="text-center" style="padding-top: 7vh">
                <h3>{"As simple as it gets"}</h3>
                <p>{"Two routes for everyhing you need"}</p>
                <br />
                <div class="row">
                    <div class="col-6">
                        <code>{"/news"}</code>
                        <p>
                            {"Get news, with 5 params to filter for your needs"}
                        </p>
                    </div>
                    <div class="col-6">
                        <code>{"/news/search"}</code>
                        <p>
                            {"Search news, either based on title or content"}
                        </p>
                    </div>
                </div>
            </div>
            <GetNews />
        </div>
    }
}
