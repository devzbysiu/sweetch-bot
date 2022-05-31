use crate::cfg::WatchedGame;
use anyhow::Result;
use log::{debug, error, info};
use serde::Deserialize;

pub(crate) fn acceptable_games<F>(watched_games: &[WatchedGame], games_provider: F) -> Vec<Game>
where
    F: Fn(String) -> Result<Vec<Game>>,
{
    info!("checking games on sale");
    let mut games = Vec::new();
    for watched_game in watched_games {
        let found_games = match games_provider(watched_game.title()) {
            Ok(games) => games,
            Err(e) => {
                error!("failed to fetch games: {}", e);
                vec![]
            }
        };
        debug!("found games: {:#?}", &found_games);
        games.extend(filter(found_games, watched_game));
    }
    games
}

pub(crate) fn fetch<S: Into<String>>(title: S) -> Result<Vec<Game>> {
    let root = ureq::get(&build_url(title)).call()?.into_json::<Root>()?;
    Ok(root.response.docs)
}

fn build_url<S: Into<String>>(title: S) -> String {
    let title = title.into();
    let title_normalized = title.replace(':', "\\:"); // normalization because of solr used underneath

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
        .filter(|game| {
            if watched_game.acceptable_price().is_some() {
                price_acceptable(game, watched_game)
            } else {
                is_on_sale(game)
            }
        })
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
        result = check_price_acceptable(game, price);
    }
    debug!("checking price is acceptable or game on sale: {}", result);
    result
}

fn check_price_acceptable(game: &Game, price: f64) -> bool {
    debug!(
        "filtering by acceptable price: checking {} <= {}",
        game.lowest_price(),
        price
    );
    game.lowest_price() <= price
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct Game {
    title: String,
    price_discounted_f: Option<f64>,
    // not all entries in API have a price
    price_regular_f: Option<f64>,
    price_has_discount_b: Option<bool>,
}

impl Game {
    #[cfg(test)]
    pub(crate) fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            ..Game::default()
        }
    }

    pub(crate) fn title(&self) -> String {
        self.title.clone()
    }

    fn lowest_price(&self) -> f64 {
        let price_regular = self.price_regular_f.unwrap_or(f64::MAX);
        match self.price_discounted_f {
            Some(discounted_price) => discounted_price.min(price_regular),
            None => price_regular,
        }
    }

    fn is_on_sale(&self) -> bool {
        self.price_has_discount_b.unwrap_or(false)
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
    fn test_acceptable_games_with_empty_games_list() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![]);

        let watched_games = vec![WatchedGame::new("Game 1").with_acceptable_price(1.0)];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    fn provider_with_ok_result(result: Vec<Game>) -> Box<dyn Fn(String) -> Result<Vec<Game>>> {
        Box::new(move |_| -> Result<Vec<Game>> { Ok(result.clone()) })
    }

    #[test]
    fn test_acceptable_price_with_title_not_matching() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 3")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_acceptable_games_with_regular_price_acceptable() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_regular_f: Some(7.0),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 1").with_acceptable_price(10.0)];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_regular_f: Some(7.0),
                ..Game::default()
            }]
        );
    }

    #[test]
    fn test_acceptable_games_with_discounted_price_acceptable() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_discounted_f: Some(5.0),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 1").with_acceptable_price(10.0)];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_discounted_f: Some(5.0),
                ..Game::default()
            }]
        );
    }

    #[test]
    fn test_acceptable_games_with_game_which_is_on_sale() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(true),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(true),
                ..Game::default()
            }]
        );
    }

    #[test]
    fn test_acceptable_games_without_acceptable_price_but_with_discount() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(true),
                price_discounted_f: Some(150.0),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(true),
                price_discounted_f: Some(150.0),
                ..Game::default()
            }]
        );
    }

    #[test]
    fn test_acceptable_games_with_acceptable_price_and_without_discount() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(false),
                price_regular_f: Some(5.0),
                ..Game::default()
            },
            Game::default(),
        ]);

        let watched_games = vec![WatchedGame::new("Game 1").with_acceptable_price(10.0)];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(
            filtered_games,
            vec![Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(false),
                price_regular_f: Some(5.0),
                ..Game::default()
            }]
        );
    }

    #[test]
    fn test_acceptable_games_without_acceptable_price_and_without_discount() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
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
        ]);

        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_acceptable_games_with_games_with_discount_set_to_false() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_has_discount_b: Some(false),
                ..Game::default()
            },
            Game {
                title: "Game 2".into(),
                price_has_discount_b: Some(false),
                ..Game::default()
            },
        ]);

        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_acceptable_games_with_games_without_discount_field() {
        testutils::setup_logger();
        // given
        let games_provider = provider_with_ok_result(vec![
            Game {
                title: "Game 1".into(),
                price_has_discount_b: None,
                ..Game::default()
            },
            Game {
                title: "Game 2".into(),
                price_has_discount_b: None,
                ..Game::default()
            },
        ]);

        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_acceptable_games_when_provider_returns_error() {
        testutils::setup_logger();
        // given
        let games_provider = |_| -> Result<Vec<Game>> { anyhow::bail!("Some error") };
        let watched_games = vec![WatchedGame::new("Game 1")];

        // when
        let filtered_games = acceptable_games(&watched_games, games_provider);

        // then
        assert_eq!(filtered_games, vec![]);
    }

    #[test]
    fn test_game_default() {
        testutils::setup_logger();
        // given
        let game = Game {
            title: "".into(),
            price_discounted_f: None,
            price_regular_f: None,
            price_has_discount_b: None,
        };

        // when
        let default = Game::default();

        // then
        assert_eq!(game, default);
    }
}
