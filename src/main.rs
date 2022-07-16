use crate::cfg::{config_path, Config};
use crate::init::handle_args;
use crate::notifier::{notify_failure, notify_success};
use crate::switch::{acceptable_games, fetch};

use anyhow::Result;
use rutils::file_logger::setup_logger;
use std::env;
use std::fs::read_to_string;

mod cfg;
mod init;
mod notifier;
mod switch;
#[cfg(test)]
mod testutils;

fn main() -> Result<()> {
    handle_args(&env::args().collect::<Vec<String>>())?;

    let config_content = read_to_string(config_path())?;
    setup_logger()?;

    let games_cfg = Config::load(&config_content)?;
    check_games_on_sale(games_cfg)?;
    Ok(())
}

fn check_games_on_sale(games_cfg: Config) -> Result<()> {
    let games = acceptable_games(&games_cfg.watched_games(), fetch);
    if games.is_empty() {
        notify_failure()?;
    } else {
        notify_success(&games)?;
    }
    Ok(())
}
