use crate::entities::WatchedGame;

use anyhow::Result;
use log::debug;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "watched_game")]
    watched_games: Vec<WatchedGame>,
}

impl Config {
    pub fn load(content: &str) -> Result<Self> {
        let cfg: Config = toml::from_str(content)?;
        debug!("loaded config: {:#?}", cfg);
        Ok(cfg)
    }

    pub fn watched_games(&self) -> Vec<WatchedGame> {
        self.watched_games.clone()
    }
}

pub fn config_path() -> PathBuf {
    sweetch_dir().join("sweetch-bot.toml")
}

pub fn sweetch_dir() -> PathBuf {
    dirs::config_dir()
        .expect("failed to read config dir while init")
        .join("sweetch-bot")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils;

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
        let watched_games_cfg = Config::load(config_content).unwrap();

        // then
        assert_eq!(watched_games_cfg.watched_games().len(), 2);
        assert_eq!(
            watched_games_cfg.watched_games(),
            vec![
                WatchedGame::new("Game 1 title here").with_acceptable_price(0.7),
                WatchedGame::new("Game 2 title here"),
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
        let watched_games_cfg = Config::load(config_content).unwrap();

        // then
        assert_eq!(watched_games_cfg.watched_games().len(), 2);
        assert_eq!(
            watched_games_cfg.watched_games(),
            vec![
                WatchedGame::new("Game 1 title here").with_acceptable_price(0.7),
                WatchedGame::new("Game 2 title here"),
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
        let _not_important = Config::load(config_content).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_watched_games_are_mandatory() {
        testutils::setup_logger();
        // given
        let config_content = "";

        // should_panic
        let _not_important = Config::load(config_content).unwrap();
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
