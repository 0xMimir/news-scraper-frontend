use chrono::{NaiveDateTime};
use yew::{function_component, html, Properties, Html};

use crate::services::NewsEntry;

#[derive(Properties, PartialEq, Eq)]
pub struct Props{
    pub entry: NewsEntry
}

#[function_component(ShowNewsEntry)]
pub fn news_entry(props: &Props) -> Html{
    let entry = props.entry.clone();

	let naive = NaiveDateTime::from_timestamp_opt(entry.released_at_unix, 0).unwrap();
	let published_at = naive.format("%Y/%m/%d %H:%M")
		.to_string();

	let valid_image = entry.image.is_some();

    html!{
        <div class="row news-entry">
            <div class={format!("col-{} text-justify", if valid_image {8} else {12})}>
				<a href={entry.url.clone()} target="_blank" rel="noreferrer noopener">
                	<h5>{entry.title.clone()}</h5>
				</a>
				<p>{format!("Published: {}", published_at)}</p>
				<p>{entry.description.clone()}</p>
            </div>
            {if let Some(img) = entry.image{
                html!{
                    <div class="col-4">
                        <a href={entry.url.clone()} target="_blank" rel="noreferrer noopener">
							<img class="news-image" src={img} alt={entry.title.clone()}/>
						</a>
                    </div>
                }
            }else{
                html!{
                    <></>
                }
            }}
        </div>
    }
}

impl NewsEntry{
    pub fn to_html(&self) -> Html{
        html!{
            <ShowNewsEntry entry={self.clone()} />
        }
    }
}