use thousands::Separable;
use yew::{function_component, html, platform::spawn_local, use_state_eq, Html};

use crate::{components::ShowNews, store::news::NewsStore};

#[function_component(GetNews)]
pub fn get_news() -> Html {
    // let state = use_async(async move { NewsStore::get_news().await });

    // if use_is_first_mount() {
    //     state.run();
    // }

    let state = use_state_eq(|| None);
    {
        let state = state.clone();
        if state.is_none() {
            spawn_local(async move {
                let r = match NewsStore::get_news().await {
                    Ok(response) => Some(response),
                    Err(_) => None,
                };
                state.set(r)
            });
        }
    }

    html! {
        <div class="text-center" style="padding-top: 7vh">
            <h3>{"Get latest news"}</h3>
            {
                if let Some(response) = &*state{
                    let news_count = (response.total / 1000) * 1000;
                    html!{
                        <>
                            <p>{format!("Over {}+ news entires", news_count.separate_with_commas())}</p>
                            <ShowNews news={response.items.clone()} />
                        </>
                    }
                }else{
                    html!{}
                }
            }
        </div>
    }
}
