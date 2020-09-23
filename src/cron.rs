use anyhow::Result;
use clokwerk::{Scheduler, TimeUnits};
use log::debug;
use std::thread;
use std::time::Duration;

pub(crate) fn schedule<F>(mut fun: F) -> !
where
    F: 'static + FnMut() -> Result<()> + Send,
{
    let mut scheduler = Scheduler::new();
    scheduler
        .every(1.day())
        .at("6:25 pm")
        .run(move || fun().expect("failed to run fun in scheduler"));
    run(&mut scheduler);
}

fn run(scheduler: &mut Scheduler) -> ! {
    loop {
        scheduler.run_pending();
        debug!("sleeping");
        thread::sleep(Duration::from_secs(10));
    }
}
