use crate::cfg::WatchedGame;
use anyhow::Result;
use log::{debug, error, info};
use serde::Deserialize;

pub(crate) fn acceptable_games(watched_games: &[WatchedGame]) -> Result<Vec<Game>> {
    info!("checking games on sale");
    let mut games = Vec::new();
    for watched_game in watched_games {
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
        .filter(|game| price_acceptable(game, watched_game) || is_on_sale(game))
        .collect::<Vec<Game>>()
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

fn price_acceptable(game: &Game, watched_game: &WatchedGame) -> bool {
    let mut result = false;
    if let Some(price) = watched_game.acceptable_price() {
        result = check_price_acceptable(&game, price);
    }
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

fn is_on_sale(game: &Game) -> bool {
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

#[derive(Deserialize, Debug, PartialEq)]
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
    use crate::testutils;

    #[test]
    fn test_build_url_with_space() {
        testutils::setup_logger();
        // given
        let title = "Test title";

        // when
        let url = build_url(title);

        // then
        assert_eq!(
            url,
            "http://search.nintendo-europe.com/en/select?rows=99\
            &fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND\
            %20product_code_txt:*%20AND%20title:Test title&q=Test title\
            &sort=sorting_title%20asc&start=0&wt=json"
        );
    }

    #[test]
    fn test_build_url_with_colon() {
        testutils::setup_logger();
        // given
        let title = "Test:title";

        // when
        let url = build_url(title);

        // then
        assert_eq!(
            url,
            "http://search.nintendo-europe.com/en/select?rows=99\
            &fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND\
            %20product_code_txt:*%20AND%20title:Test\\:title&q=Test:title\
            &sort=sorting_title%20asc&start=0&wt=json"
        );
    }

    #[test]
    fn test_build_url_with_special_chars() {
        testutils::setup_logger();
        // given
        let title = "Test!@#$%^&*()-=[]\\;',./<>?:\"{}|+_";

        // when
        let url = build_url(title);

        // then
        assert_eq!(
            url,
            "http://search.nintendo-europe.com/en/select?rows=99\
            &fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND\
            %20product_code_txt:*%20AND%20title:Test!@#$%^&*()-=[]\\;\
            ',./<>?\\:\"{}|+_&q=Test!@#$%^&*()-=[]\\;',./<>?:\"{}|+_\
            &sort=sorting_title%20asc&start=0&wt=json"
        );
    }

    #[test]
    fn test_build_url_with_empty_title() {
        testutils::setup_logger();
        // given
        let title = "";

        // when
        let url = build_url(title);

        // then
        assert_eq!(
            url,
            "http://search.nintendo-europe.com/en/select?rows=99\
            &fq=type:GAME%20AND%20system_type:nintendoswitch*%20AND\
            %20product_code_txt:*%20AND%20title:&q=&sort=sorting_title\
            %20asc&start=0&wt=json"
        );
    }

    #[test]
    fn test_filter_with_one_game_matched() {
        testutils::setup_logger();
        // given
        let games = vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
            Game {
                title: "Game 2".into(),
                price_discounted_f: Some(7.0),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
        ];

        let watched_game = WatchedGame::new("Game 1").with_acceptable_price(1.0);

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            }]
        );
    }

    #[test]
    fn test_filter_with_title_not_matching() {
        testutils::setup_logger();
        // given
        let games = vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
            Game {
                title: "Game 2".into(),
                price_discounted_f: Some(7.0),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
        ];

        let watched_game = WatchedGame::new("Game 3");

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_filter_with_empty_games_list() {
        testutils::setup_logger();
        // given
        let games = vec![];

        let watched_game = WatchedGame::new("Game 1").with_acceptable_price(1.0);

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_filter_without_acceptable_price_but_with_discount() {
        testutils::setup_logger();
        // given
        let games = vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
            Game {
                title: "Game 2".into(),
                price_discounted_f: Some(7.0),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            },
        ];

        let watched_game = WatchedGame::new("Game 1");

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(true),
            }]
        );
    }

    #[test]
    fn test_filter_without_acceptable_price_and_without_discount() {
        testutils::setup_logger();
        // given
        let games = vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(false),
            },
            Game {
                title: "Game 2".into(),
                price_discounted_f: Some(7.0),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(false),
            },
        ];

        let watched_game = WatchedGame::new("Game 1");

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_filter_with_games_without_price() {
        testutils::setup_logger();
        // given
        let games = vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(0.5),
                price_regular_f: None,
                price_has_discount_b: Some(false),
            },
            Game {
                title: "Game 2".into(),
                price_discounted_f: Some(7.0),
                price_regular_f: Some(7.0),
                price_has_discount_b: Some(false),
            },
        ];

        let watched_game = WatchedGame::new("Game 1");

        // when
        let filtered_games = filter(games, &watched_game);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_game_has_price_when_price_discounted_set() {
        testutils::setup_logger();
        // given
        let game = Game {
            title: "Not important".into(),
            price_discounted_f: Some(1.0),
            ..Game::default()
        };

        // when
        let has_price = game.has_price();

        // then
        assert_eq!(has_price, true);
    }

    #[test]
    fn test_game_has_price_when_price_regular_set() {
        testutils::setup_logger();
        // given
        let game = Game {
            title: "Not important".into(),
            price_regular_f: Some(1.0),
            ..Game::default()
        };

        // when
        let has_price = game.has_price();

        // then
        assert_eq!(has_price, true);
    }

    #[test]
    fn test_games_has_price_when_price_not_set() {
        testutils::setup_logger();
        // given
        let game = Game::default();

        // when
        let has_price = game.has_price();

        // then
        assert_eq!(has_price, false);
    }
}
