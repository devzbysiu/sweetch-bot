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
        let cfg: T = toml::from_str(&content)?;
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
        match self.debug {
            Some(debug) => debug,
            None => false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ScheduleConfig {
    schedule: Schedule,
}

impl ScheduleConfig {
    pub(crate) fn schedule(&self) -> Vec<String> {
        self.schedule.run_at.clone()
    }
}

pub(crate) fn config_path() -> Result<PathBuf> {
    Ok(sweetch_dir()?.join("sweetch-bot.toml"))
}

#[derive(Debug, Clone, Deserialize)]
struct Schedule {
    run_at: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WatchedGamesConfig {
    schedule: Schedule,
    #[serde(rename = "watched_game")]
    watched_games: Vec<WatchedGame>,
}

impl WatchedGamesConfig {
    pub(crate) fn watched_games(&self) -> Vec<WatchedGame> {
        self.watched_games.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
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
        assert_eq!(dbg_config.debug_enabled(), false);
    }

    #[test]
    fn test_load_debug_config_with_debug_option_set_to_false() {
        testutils::setup_logger();
        // given
        let config_content = "debug = false";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.debug_enabled(), false);
    }

    #[test]
    fn test_load_debug_config_with_debug_enabled() {
        testutils::setup_logger();
        // given
        let config_content = "debug = true";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.debug_enabled(), true);
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
        assert_eq!(dbg_config.debug_enabled(), true);
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
        assert_eq!(dbg_config.debug_enabled(), false);
    }

    #[test]
    fn test_load_debug_config_with_empty_config() {
        testutils::setup_logger();
        // given
        let config_content = "";

        // when
        let dbg_config = Config::load::<DebugConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.debug_enabled(), false);
    }

    #[test]
    fn test_load_schedule_config_with_one_hour() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [schedule]
            run_at = ["7:00 pm"]
        "#;

        // when
        let dbg_config = Config::load::<ScheduleConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.schedule(), vec!["7:00 pm"]);
    }

    #[test]
    fn test_load_schedule_config_with_multiple_hours() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [schedule]
            run_at = ["7:00 am", "6:00 pm"]
        "#;

        // when
        let dbg_config = Config::load::<ScheduleConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.schedule(), vec!["7:00 am", "6:00 pm"]);
    }

    #[test]
    fn test_load_schedule_config_without_hours() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [schedule]
            run_at = []
        "#;

        // when
        let dbg_config = Config::load::<ScheduleConfig>(config_content).unwrap();

        // then
        assert_eq!(dbg_config.schedule(), Vec::<String>::new());
    }

    #[test]
    fn test_load_schedule_config_with_wrong_inputs() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [schedule]
            run_at = ["incorrect hour 1", "incorrect hour 2"]
        "#;

        // when
        let dbg_config = Config::load::<ScheduleConfig>(config_content).unwrap();

        // then
        assert_eq!(
            dbg_config.schedule(),
            vec!["incorrect hour 1", "incorrect hour 2"]
        );
    }

    #[test]
    #[should_panic]
    fn test_schedule_is_mandatory() {
        testutils::setup_logger();
        // given
        let config_content = r#"
            [[watched_game]]
            title = "Game 1 title here"

            [[watched_game]]
            title = "Game 2 title here"
         "#;

        // should panic
        let _ = Config::load::<ScheduleConfig>(config_content).unwrap();
    }
}
