use anyhow::Result;
use cfg::{Config, DebugConfig, ScheduleConfig, WatchedGamesConfig};
use cron::schedule;
use daemon::daemonize;
use flexi_logger::{detailed_format, Age, Cleanup, Criterion, Logger, Naming};
use init::{handle_args, sweetch_dir};
use log::debug;
use notifier::{notify_failure, notify_success};
use std::env;
use switch::acceptable_games;

mod cfg;
mod cron;
mod daemon;
mod init;
mod notifier;
mod switch;

fn main() -> Result<()> {
    handle_args(&env::args().collect::<Vec<String>>())?;
    daemonize(|| -> Result<()> {
        setup_logger()?;
        debug!("starting bot");
        let cfg = Config::load::<ScheduleConfig>()?;
        schedule(cfg.schedule(), || -> Result<()> {
            let games_cfg = Config::load::<WatchedGamesConfig>()?;
            let games = acceptable_games(games_cfg.watched_games())?;
            if games.len() > 0 {
                notify_success(games)?;
            } else {
                notify_failure()?;
            }
            Ok(())
        });
    })?;
    Ok(())
}

fn setup_logger() -> Result<()> {
    let debug_cfg = Config::load::<DebugConfig>()?;
    let log_str = match debug_cfg.debug_enabled() {
        true => "sweetch_bot=debug",
        false => "sweetch_bot=info",
    };
    Logger::with_str(log_str)
        .directory(sweetch_dir()?)
        .log_to_file()
        .format(detailed_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3),
        )
        .start()?;
    Ok(())
}
