use anyhow::Result;
use cfg::{Config, DebugConfig, ScheduleConfig, WatchedGamesConfig};
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
    match debug_cfg.debug_enabled() {
        true => std::env::set_var("RUST_LOG", "sweetch_bot=debug"),
        false => std::env::set_var("RUST_LOG", "sweetch_bot=info"),
    }
    pretty_env_logger::init_timed();
    Ok(())
}
