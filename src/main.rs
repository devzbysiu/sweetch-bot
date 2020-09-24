use anyhow::Result;
use cron::schedule;
use daemon::daemonize;
use log::debug;
use notifier::{notify_failure, notify_success};
use switch::games_on_sale;

mod cron;
mod daemon;
mod notifier;
mod switch;

fn main() -> Result<()> {
    daemonize(|| -> Result<()> {
        setup_logger();
        debug!("starting bot");
        schedule(|| -> Result<()> {
            let games = games_on_sale()?;
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
