use anyhow::Result;
use cron::schedule;
use daemon::daemonize;
use log::debug;
use notifier::{notify_failure, notify_success};
use switch::are_games_on_sale;

mod cron;
mod daemon;
mod notifier;
mod switch;

fn main() -> Result<()> {
    daemonize(|| -> Result<()> {
        std::env::set_var("RUST_LOG", "debug");
        pretty_env_logger::init_timed();
        debug!("starting bot");
        schedule(|| -> Result<()> {
            if are_games_on_sale()? {
                notify_success()?;
            } else {
                notify_failure()?;
            }
            Ok(())
        });
    })?;
    Ok(())
}
