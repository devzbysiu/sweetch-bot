use anyhow::Result;
use notify_rust::{Notification, Timeout, Urgency};

pub(crate) fn notify_success() -> Result<()> {
    println!("found games on sale - sending notification");
    Notification::new()
        .summary("Game Available")
        .body("New Switch game is on sale! Check it out.")
        .timeout(Timeout::Never)
        .urgency(Urgency::Critical)
        .show()?;
    Ok(())
}

pub(crate) fn notify_failure() -> Result<()> {
    println!("no games on sale found - sending notification");
    Notification::new()
        .summary("No Gamges")
        .body("No games on sale found.")
        .timeout(Timeout::from(10_000))
        .urgency(Urgency::Low)
        .show()?;
    Ok(())
}
