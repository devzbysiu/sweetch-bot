use anyhow::Result;
use log::info;
use serde::Deserialize;
use ureq;

pub(crate) fn games_on_sale() -> Result<Vec<Game>> {
    info!("checking games on sale");
    Ok(fetch()?)
}

fn fetch() -> Result<Vec<Game>> {
    let root: Root = ureq::get("http://search.nintendo-europe.com/en/select?rows=99&fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND%20product_code_txt:*&q=Ori&sort=sorting_title%20asc&start=0&wt=json")
        .call()
        .into_json_deserialize::<Root>()?;
    Ok(root.response.docs)
}

#[derive(Deserialize)]
struct Root {
    response: Response,
}

#[derive(Deserialize)]
struct Response {
    docs: Vec<Game>,
}

#[derive(Deserialize)]
pub(crate) struct Game {
    pub(crate) title: String,
}
