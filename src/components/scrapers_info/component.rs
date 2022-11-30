use yew::{function_component, html, Html, Properties};
use yew_hooks::{use_async, use_interval, use_is_first_mount};

use crate::services::{AdminStore, ScraperInfo};

#[derive(Properties, PartialEq)]
struct RowParams {
    row_data: ScraperInfo,
}

#[function_component(TableRow)]
fn table_row(props: &RowParams) -> Html {
    html! {
        <tr>
            <th>{&props.row_data.blog_id}</th>
            <td>{&props.row_data.total}</td>
            <td>{&props.row_data.scraped}</td>
            <td>{&props.row_data.unscraped}</td>
            <td>{&props.row_data.deleted}</td>
            <td>{&props.row_data.processed}</td>
            <td>{&props.row_data.error}</td>
        </tr>
    }
}

#[function_component(ScrapersInfo)]
pub fn scrapers_info() -> Html {
    let state = use_async(async move { AdminStore::get_scraper_info().await });

    if use_is_first_mount() {
        state.run();
    }

    {
        let state = state.clone();
        use_interval(move || state.run(), 2000)
    }

    let sum_row = if let Some(news) = &state.data {
        let f = news
            .iter()
            .map(|x| {
                (
                    x.total,
                    x.scraped,
                    x.unscraped,
                    x.deleted,
                    x.processed,
                    x.error,
                )
            })
            .collect::<Vec<(i32, i32, i32, i32, i32, i32)>>();

        let total: i32 = f.iter().map(|x| x.0).sum();
        let scraped: i32 = f.iter().map(|x| x.1).sum();
        let unscraped: i32 = f.iter().map(|x| x.2).sum();
        let deleted: i32 = f.iter().map(|x| x.3).sum();
        let processed: i32 = f.iter().map(|x| x.4).sum();
        let error: i32 = f.iter().map(|x| x.5).sum();

        ScraperInfo {
            blog_id: "Total: ".to_owned(),
            total,
            scraped,
            unscraped,
            deleted,
            processed,
            error,
        }
    } else {
        ScraperInfo {
            blog_id: "Total: ".to_owned(),
            total: 0,
            scraped: 0,
            unscraped: 0,
            deleted: 0,
            processed: 0,
            error: 0,
        }
    };

    html! {
        <>
            <h4>{"Scrapers info:"}</h4>
            <table class="table table-dark">
                <thead class="thead-dark">
                    <tr>
                        <th scope="col">{"Scraper"}</th>
                        <th scope="col">{"Total"}</th>
                        <th scope="col">{"Scraped"}</th>
                        <th scope="col">{"Unscraped"}</th>
                        <th scope="col">{"Deleted"}</th>
                        <th scope="col">{"Processed"}</th>
                        <th scope="col">{"Errors"}</th>
                    </tr>
                </thead>
                <tbody>
                    {if let Some(news) = &state.data{
                        news.iter().map(|x| html!{
                            <TableRow row_data={x.clone()} />
                        }).collect::<Html>()
                    }else{
                        html!{}
                    }}
                    <TableRow row_data={sum_row} />
                </tbody>
            </table>
        </>
    }
}