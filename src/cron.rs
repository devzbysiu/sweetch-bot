use anyhow::Result;
use clokwerk::{Scheduler, TimeUnits};
use log::{debug, error, info};
use std::thread;
use std::time::Duration;

pub(crate) fn schedule<F>(mut fun: F) -> !
where
    F: 'static + FnMut() -> Result<()> + Send,
{
    let mut scheduler = Scheduler::new();
    scheduler
        .every(1.day())
        .at("7:18 pm")
        .and_every(1.day())
        .at("7:19 pm")
        .run(move || fun().unwrap_or_else(|_| error!("failed to run fun in scheduler")));
    debug!("starting scheduler");
    run(&mut scheduler);
}

fn run(scheduler: &mut Scheduler) -> ! {
    let seconds = 10;
    loop {
        scheduler.run_pending();
        info!("no time yet, waiting {} seconds", seconds);
        thread::sleep(Duration::from_secs(seconds));
    }
}
