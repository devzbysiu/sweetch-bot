use crate::cfg::{config_path, Config, DebugConfig, ScheduleConfig, WatchedGamesConfig};
use crate::init::{handle_args, sweetch_dir};
use crate::notifier::{notify_failure, notify_success};
use crate::switch::{acceptable_games, fetch};

use anyhow::Result;
use flexi_logger::{detailed_format, Age, Cleanup, Criterion, FileSpec, Logger, Naming};
use log::debug;
use rutils::daemon::daemonize;
use rutils::scheduler::schedule;
use std::env;
use std::fs::read_to_string;

mod cfg;
mod init;
mod notifier;
mod switch;
#[cfg(test)]
mod testutils;

// TODO: get rid of unwraps
fn main() -> Result<()> {
    handle_args(&env::args().collect::<Vec<String>>())?;
    daemonize(|| {
        let config_content = read_to_string(config_path()).unwrap();
        setup_logger(&config_content).unwrap();
        debug!("starting bot");
        let cfg = Config::load::<ScheduleConfig>(&config_content).unwrap();
        schedule(&cfg.schedule(), || {
            let config_content = read_to_string(config_path()).unwrap();
            let games_cfg = Config::load::<WatchedGamesConfig>(&config_content).unwrap();
            let games = acceptable_games(&games_cfg.watched_games(), fetch);
            if games.is_empty() {
                notify_failure().unwrap();
            } else {
                notify_success(&games).unwrap();
            }
        });
    })?;
    Ok(())
}

fn setup_logger(config: &str) -> Result<()> {
    let debug_cfg = Config::load::<DebugConfig>(config)?;
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
