use yew::{function_component, html, Component, Html, Properties};

use crate::{
    store::{admin::AdminStore, objects::scraper_info::ScraperInfo},
    Error,
};

#[derive(Properties, PartialEq)]
struct RowParams {
    row_data: ScraperInfo,
}

#[function_component(TableRow)]
fn table_row(props: &RowParams) -> Html {
    let total: f64 = props.row_data.total.into();
    let css_class = match Into::<f64>::into(props.row_data.error) / total > 0.001 {
        true => "bg-danger",
        false => "",
    };

    html! {
        <tr class={css_class}>
            <td>{&props.row_data.blog_id}</td>
            <td>{format!("{}", props.row_data.total)}</td>
            <td>{
                if props.row_data.scraped == 0{
                    "0".to_owned()
                }else{
                    format!("{}({:.2}%)", props.row_data.scraped, (Into::<f64>::into(props.row_data.scraped) / total * 100.0))
                }
            }</td>
            <td>{
                if props.row_data.unscraped == 0{
                    "0".to_owned()
                }else{
                    format!("{}({:.2}%)", props.row_data.unscraped, (Into::<f64>::into(props.row_data.unscraped) / total * 100.0))
                }
            }</td>
            <td>{
                if props.row_data.deleted == 0{
                    "0".to_owned()
                }else{
                    format!("{}({:.2}%)", props.row_data.deleted, (Into::<f64>::into(props.row_data.deleted) / total * 100.0))
                }
            }</td>
            <td>{
                if props.row_data.processed == 0{
                    "0".to_owned()
                }else{
                    format!("{}({:.2}%)", props.row_data.processed, (Into::<f64>::into(props.row_data.processed) / total * 100.0))
                }
            }</td>
            <td>{
                if props.row_data.error == 0{
                    "0".to_owned()
                }else{
                    format!("{}({:.2}%)", props.row_data.error, (Into::<f64>::into(props.row_data.error) / total * 100.0))
                }
            }</td>
        </tr>
    }
}

fn sum_row(scrapers_info: &[ScraperInfo]) -> ScraperInfo {
    let f = scrapers_info
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
}

pub struct ScrapersInfo {
    scrapers_info: Option<Vec<ScraperInfo>>,
}

impl Component for ScrapersInfo {
    type Message = Result<Vec<ScraperInfo>, Error>;
    type Properties = ();
    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future(AdminStore::get_scraper_info());

        let callback = ctx.link().callback(Result::Ok);
        AdminStore::update_scraper_info(callback);

        Self {
            scrapers_info: None,
        }
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.scrapers_info = match msg {
            Ok(update) => Some(update),
            Err(_) => None,
        };

        true
    }
    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        match &self.scrapers_info {
            Some(scrapers) => {
                html! {
                    <table class="table table-dark table-fixed">
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
                            {
                                scrapers.iter().map(|x| html!{
                                    <TableRow row_data={x.clone()} />
                                }).collect::<Html>()
                            }
                        </tbody>
                        <tfoot class="thead-dark">
                            <TableRow row_data={sum_row(scrapers)} />
                        </tfoot>
                    </table>
                }
            }
            None => html! {},
        }
    }
}
