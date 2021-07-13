use anyhow::Result;
use cfg::{config_path, Config, DebugConfig, ScheduleConfig, WatchedGamesConfig};
use cron::schedule;
use daemon::daemonize;
use flexi_logger::FileSpec;
use flexi_logger::{detailed_format, Age, Cleanup, Criterion, Logger, Naming};
use init::{handle_args, sweetch_dir};
use log::debug;
use notifier::{notify_failure, notify_success};
use std::env;
use std::fs::read_to_string;
use switch::acceptable_games;
use switch::fetch;

mod cfg;
mod cron;
mod daemon;
mod init;
mod notifier;
mod switch;
#[cfg(test)]
mod testutils;

fn main() -> Result<()> {
    handle_args(&env::args().collect::<Vec<String>>())?;
    daemonize(|| -> Result<()> {
        let config_content = read_to_string(config_path())?;
        setup_logger(&config_content)?;
        debug!("starting bot");
        let cfg = Config::load::<ScheduleConfig>(&config_content)?;
        schedule(&cfg.schedule(), || -> Result<()> {
            let config_content = read_to_string(config_path())?;
            let games_cfg = Config::load::<WatchedGamesConfig>(&config_content)?;
            let games = acceptable_games(&games_cfg.watched_games(), fetch);
            if games.is_empty() {
                notify_failure()?;
            } else {
                notify_success(&games)?;
            }
            Ok(())
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
