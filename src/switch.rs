use crate::cfg::WatchedGame;
use anyhow::Result;
use log::{debug, info};
use serde::Deserialize;
use ureq;

pub(crate) fn acceptable_games(watched_games: Vec<WatchedGame>) -> Result<Vec<Game>> {
    info!("checking games on sale");
    let mut games = Vec::new();
    for watched_game in &watched_games {
        let found_games = fetch(watched_game.title())?;
        debug!("found games: {:#?}", &found_games);
        games.extend(filter(
            found_games,
            &watched_game.title(),
            watched_game.acceptable_price(),
        ));
    }
    Ok(games)
}

fn fetch<S: Into<String>>(title: S) -> Result<Vec<Game>> {
    let root: Root = ureq::get(&build_url(title))
        .call()
        .into_json_deserialize::<Root>()?;
    Ok(root.response.docs)
}

fn build_url<S: Into<String>>(title: S) -> String {
    let title = title.into();
    let url =
    format!("http://search.nintendo-europe.com/en/select?rows=99&fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND%20product_code_txt:*%20AND%20title:{}&q={}&sort=sorting_title%20asc&start=0&wt=json", title, title);
    debug!("built url: {}", url);
    url
}

fn filter(games: Vec<Game>, title: &str, price: f64) -> Vec<Game> {
    debug!("filtering by title: {}", title);
    games
        .into_iter()
        .filter(|game| game.title() == title)
        .filter(|game| game.price() <= price)
        .collect::<Vec<Game>>()
}

#[derive(Deserialize)]
struct Root {
    response: Response,
}

#[derive(Deserialize)]
struct Response {
    docs: Vec<Game>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Game {
    title: String,
    price_discounted_f: Option<f64>,
    price_regular_f: f64,
}

impl Game {
    pub(crate) fn title(&self) -> String {
        self.title.clone()
    }

    fn price(&self) -> f64 {
        match self.price_discounted_f {
            Some(discounted_price) => discounted_price.min(self.price_regular_f),
            None => self.price_regular_f,
        }
    }
}
