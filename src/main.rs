use crate::cfg::{config_path, Config, DebugConfig, WatchedGamesConfig};
use crate::init::{handle_args, sweetch_dir};
use crate::notifier::{notify_failure, notify_success};
use crate::switch::{acceptable_games, fetch};

use anyhow::Result;
use flexi_logger::{detailed_format, Age, Cleanup, Criterion, FileSpec, Logger, Naming};
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
    let debug_cfg = Config::load::<DebugConfig>(&config_content)?;
    setup_logger(debug_cfg)?;

    let games_cfg = Config::load::<WatchedGamesConfig>(&config_content)?;
    check_games_on_sale(games_cfg)?;
    Ok(())
}

fn setup_logger(debug_cfg: DebugConfig) -> Result<()> {
    let log_str = if debug_cfg.debug_enabled() {
        "sweetch_bot=debug"
    } else {
        "sweetch_bot=info"
    };
    Logger::try_with_str(log_str)?
        .log_to_file(FileSpec::default().directory(sweetch_dir()))
        .format(detailed_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .start()?;
    Ok(())
}

fn check_games_on_sale(games_cfg: WatchedGamesConfig) -> Result<()> {
    let games = acceptable_games(&games_cfg.watched_games(), fetch);
    if games.is_empty() {
        notify_failure()?;
    } else {
        notify_success(&games)?;
    }
    Ok(())
}
