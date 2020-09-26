use anyhow::Result;
use cfg::{ScheduleConfig, WatchedGamesConfig};
use cron::schedule;
use daemon::daemonize;
use log::debug;
use notifier::{notify_failure, notify_success};
use switch::acceptable_games;

mod cfg;
mod cron;
mod daemon;
mod notifier;
mod switch;

fn main() -> Result<()> {
    daemonize(|| -> Result<()> {
        setup_logger();
        debug!("starting bot");
        let cfg = ScheduleConfig::load()?;
        schedule(cfg.schedule(), || -> Result<()> {
            let cfg = WatchedGamesConfig::load()?;
            let games = acceptable_games(cfg.watched_games())?;
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

fn setup_logger() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init_timed();
}
