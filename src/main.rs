use anyhow::Result;
use cron::schedule;
use daemon::daemonize;
use log::debug;
use notifier::notify;
use switch::are_games_on_sale;

mod cron;
mod daemon;
mod notifier;
mod switch;

fn main() -> Result<()> {
    pretty_env_logger::init();
    debug!("Starting");
    daemonize(|| -> Result<()> {
        schedule(|| -> Result<()> {
            if are_games_on_sale()? {
                notify()?;
            }
            Ok(())
        });
    })?;
    Ok(())
}
