use crate::init::sweetch_dir;
use anyhow::Result;
use log::debug;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

pub(crate) struct Config {}

impl Config {
    pub(crate) fn load<T>(content: &str) -> Result<T>
    where
        for<'a> T: Deserialize<'a> + fmt::Debug,
    {
        let cfg: T = toml::from_str(content)?;
        debug!("loaded config: {:#?}", cfg);
        Ok(cfg)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct DebugConfig {
    debug: Option<bool>,
}

impl DebugConfig {
    pub(crate) fn debug_enabled(&self) -> bool {
        self.debug.unwrap_or(false)
    }
}

pub(crate) fn config_path() -> PathBuf {
    sweetch_dir().join("sweetch-bot.toml")
}

#[derive(Debug, Deserialize)]
pub(crate) struct WatchedGamesConfig {
    #[serde(rename = "watched_game")]
    watched_games: Vec<WatchedGame>,
}

impl WatchedGamesConfig {
    pub(crate) fn watched_games(&self) -> Vec<WatchedGame> {
        self.watched_games.clone()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub(crate) struct WatchedGame {
    title: String,
    acceptable_price: Option<f64>,
}

impl WatchedGame {
    #[cfg(test)]
    pub(crate) fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            acceptable_price: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_acceptable_price(mut self, price: f64) -> Self {
        self.acceptable_price = Some(price);
        self
    }

    pub(crate) fn title(&self) -> String {
        self.title.clone()
    }

    pub(crate) fn acceptable_price(&self) -> Option<f64> {
        self.acceptable_price
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils;

    #[test]
    fn test_load_debug_config_without_debug_option() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [[watched_game]]
            title = "Game 2 title here"
        "#;

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(!dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_debug_config_with_debug_option_set_to_false() {
        testutils::setup_logger();
        // given
        let config_content = "debug = false";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(!dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_debug_config_with_debug_enabled() {
        testutils::setup_logger();
        // given
        let config_content = "debug = true";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_debug_config_with_real_life_config_and_debug_enabled() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            debug = true

            [schedule]
            run_at = ["7:00 pm"]

            [[watched_game]]
            title = "Game 1 title here"

            [[watched_game]]
            title = "Game 2 title here"
        "#;

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_debug_config_with_real_life_config_and_without_debug() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [schedule]
            run_at = ["7:00 pm"]

            [[watched_game]]
            title = "Game 1 title here"

            [[watched_game]]
            title = "Game 2 title here"
        "#;

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(!dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_debug_config_with_empty_config() {
        testutils::setup_logger();
        // given
        let config_content = "";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert!(!dbg_config.debug_enabled());
    }

    #[test]
    fn test_load_watched_games_config_with_valid_input() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [[watched_game]]
            title = "Game 1 title here"
            acceptable_price = 0.7

            [[watched_game]]
            title = "Game 2 title here"
         "#;

        // when
        let watched_games_cfg = Config::load::<WatchedGamesConfig>(config_content).unwrap();

        // then
        assert_eq!(watched_games_cfg.watched_games().len(), 2);
        assert_eq!(
            watched_games_cfg.watched_games(),
            vec![
                WatchedGame {
                    title: "Game 1 title here".into(),
                    acceptable_price: Some(0.7),
                },
                WatchedGame {
                    title: "Game 2 title here".into(),
                    acceptable_price: None,
                }
            ]
        );
    }

    #[test]
    fn test_load_watched_games_config_with_too_many_fields() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [[watched_game]]
            title = "Game 1 title here"
            acceptable_price = 0.7
            additional_field = "Something"

            [[watched_game]]
            title = "Game 2 title here"
         "#;

        // when
        let watched_games_cfg = Config::load::<WatchedGamesConfig>(config_content).unwrap();

        // then
        assert_eq!(watched_games_cfg.watched_games().len(), 2);
        assert_eq!(
            watched_games_cfg.watched_games(),
            vec![
                WatchedGame {
                    title: "Game 1 title here".into(),
                    acceptable_price: Some(0.7),
                },
                WatchedGame {
                    title: "Game 2 title here".into(),
                    acceptable_price: None,
                }
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_load_watched_games_config_without_title() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [[watched_game]]
            acceptable_price = 0.7

            [[watched_game]]
            title = "Game 2 title here"
         "#;

        // should_panic
        let _not_important = Config::load::<WatchedGamesConfig>(config_content).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_watched_games_are_mandatory() {
        testutils::setup_logger();
        // given
        let config_content = "";

        // should_panic
        let _not_important = Config::load::<WatchedGamesConfig>(config_content).unwrap();
    }

    #[test]
    fn test_config_path() {
        // given
        let config_dir = dirs::config_dir().unwrap();
        // when
        let cfg_path = config_path();

        // then
        assert_eq!(cfg_path, config_dir.join("sweetch-bot/sweetch-bot.toml"));
    }
}
