use anyhow::Result;
use dirs;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub(crate) struct ScheduleConfig {
    schedule: Schedule,
    #[serde(rename = "watched_game")]
    watched_games: Vec<WatchedGame>,
}

impl ScheduleConfig {
    pub(crate) fn load() -> Result<Self> {
        Ok(toml::from_str(&read_to_string(config_path())?)?)
    }

    pub(crate) fn schedule(&self) -> Vec<String> {
        self.schedule.run_at.clone()
    }
}

fn config_path() -> PathBuf {
    Path::new(&dirs::config_dir().expect("failed to get system configuration dir"))
        .join("sweetch-bot.toml")
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
    pub(crate) fn load() -> Result<Self> {
        Ok(toml::from_str(&read_to_string(config_path())?)?)
    }

    pub(crate) fn watched_games(&self) -> Vec<WatchedGame> {
        self.watched_games.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct WatchedGame {
    title: String,
    acceptable_price: f64,
}

impl WatchedGame {
    pub(crate) fn title(&self) -> String {
        self.title.clone()
    }

    pub(crate) fn acceptable_price(&self) -> f64 {
        self.acceptable_price
    }
}
