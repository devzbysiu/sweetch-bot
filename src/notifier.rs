use anyhow::Result;
use notify_rust::{Notification, Timeout, Urgency};

pub(crate) fn notify() -> Result<()> {
    Notification::new()
        .summary("Game Available")
        .body("New Switch game is on sale! Check it out.")
        .timeout(Timeout::Never)
        .urgency(Urgency::Critical)
        .show()?;
    Ok(())
}
