use crate::cfg::WatchedGame;
use anyhow::Result;
use log::{debug, error, info};
use serde::Deserialize;
use ureq;

pub(crate) fn acceptable_games(watched_games: Vec<WatchedGame>) -> Result<Vec<Game>> {
    info!("checking games on sale");
    let mut games = Vec::new();
    for watched_game in &watched_games {
        let found_games = match fetch(watched_game.title()) {
            Ok(games) => games,
            Err(e) => {
                error!("failed to fetch games: {}", e);
                vec![]
            }
        };
        debug!("found games: {:#?}", &found_games);
        games.extend(filter(found_games, &watched_game));
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
    let title_normalized = title.replace(":", "\\:"); // normalization because of solr used underneath

    let url = format!(
        "http://search.nintendo-europe.com/en/select?rows=99\
        &fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND\
        %20product_code_txt:*%20AND%20title:{}&q={}&sort=sorting_title\
        %20asc&start=0&wt=json",
        title_normalized, title
    );
    debug!("built url: {}", url);
    url
}

fn filter(games: Vec<Game>, watched_game: &WatchedGame) -> Vec<Game> {
    debug!("filtering by title: {}", watched_game.title());
    games
        .into_iter()
        .filter(|game| titles_match(game, watched_game))
        .filter(has_price) // for some reason, not all entries have price
        .filter(|game| price_acceptable_or_on_sale(game, watched_game))
        .collect::<Vec<Game>>()
}

fn has_price(game: &Game) -> bool {
    debug!(
        "checking game [{}] has price: {}",
        game.title(),
        game.has_price()
    );
    game.has_price()
}

fn titles_match(game: &Game, watched_game: &WatchedGame) -> bool {
    let result = game.title() == watched_game.title();
    debug!(
        "checking titles match: [{}] and [{}] => {}",
        game.title(),
        watched_game.title(),
        result
    );
    result
}

fn price_acceptable_or_on_sale(game: &Game, watched_game: &WatchedGame) -> bool {
    let result = match watched_game.acceptable_price() {
        Some(price) => check_price_acceptable(&game, price),
        None => check_is_on_sale(&game),
    };
    debug!("checking price is acceptable or game on sale: {}", result);
    result
}

fn check_price_acceptable(game: &Game, price: f64) -> bool {
    debug!(
        "filtering by acceptable price: checking {} <= {}",
        game.price(),
        price
    );
    game.price() <= price
}

fn check_is_on_sale(game: &Game) -> bool {
    debug!("filtering by 'is on sale': {}", game.is_on_sale());
    game.is_on_sale()
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
    // not all entries in API have a price
    price_regular_f: Option<f64>,
    price_has_discount_b: Option<bool>,
}

impl Game {
    pub(crate) fn title(&self) -> String {
        self.title.clone()
    }

    fn price(&self) -> f64 {
        let price_regular = self.price_regular_f.unwrap_or(f64::MAX);
        match self.price_discounted_f {
            Some(discounted_price) => discounted_price.min(price_regular),
            None => price_regular,
        }
    }

    fn is_on_sale(&self) -> bool {
        match self.price_has_discount_b {
            Some(has_discount) => has_discount,
            None => false,
        }
    }

    fn has_price(&self) -> bool {
        self.price_discounted_f.is_some() || self.price_regular_f.is_some()
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            title: "".into(),
            price_discounted_f: None,
            price_regular_f: None,
            price_has_discount_b: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_games_has_price_when_price_discounted_set() {
        // given
        let game = Game {
            title: "Not important".into(),
            price_discounted_f: Some(1.0),
            ..Default::default()
        };

        // when
        let has_price = game.has_price();

        // then
        assert_eq!(has_price, true);
    }
}
