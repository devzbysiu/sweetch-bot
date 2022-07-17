use crate::configuration::{config_path, Config};
use crate::notifier::{notify_failure, notify_success};
use crate::switch::{acceptable_games, fetch};

use anyhow::Result;
use rutils::file_logger::setup_logger;
use std::fs::read_to_string;

mod configuration;
mod data_providers;
mod entities;
mod use_cases;

mod notifier;
mod switch;
#[cfg(test)]
mod testutils;

fn main() -> Result<()> {
    setup_logger()?;
    check_games_on_sale(&Config::load(&read_to_string(config_path())?)?)?;
    Ok(())
}

fn check_games_on_sale(games_cfg: &Config) -> Result<()> {
    let games = acceptable_games(&games_cfg.watched_games(), fetch);
    if games.is_empty() {
        notify_failure()?;
    } else {
        notify_success(&games)?;
    }
    Ok(())
}
