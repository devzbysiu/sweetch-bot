use anyhow::Result;
use clokwerk::{Scheduler, TimeUnits};
use log::{debug, error, info};
use std::thread;
use std::time::Duration;

pub(crate) fn schedule<F>(schedule: Vec<String>, mut fun: F) -> !
where
    F: 'static + FnMut() -> Result<()> + Send,
{
    let mut scheduler = Scheduler::new();
    let default_time = "12:01 pm".into();
    let first_hour = schedule.first().unwrap_or(&default_time);
    info!("setting scheduler at: {}", first_hour);
    let mut job = scheduler.every(1.day()).at(first_hour);
    if schedule.len() > 1 {
        for time in &schedule[1..] {
            info!("setting scheduler at: {}", time);
            job = job.and_every(1.day()).at(&time);
        }
    }
    job.run(move || match fun() {
        Ok(_) => {}
        Err(e) => error!("failed to run fun in scheduler: {}", e),
    });
    debug!("starting scheduler");
    run(&mut scheduler);
}

fn run(scheduler: &mut Scheduler) -> ! {
    let seconds = 600;
    loop {
        scheduler.run_pending();
        info!("no time yet, waiting {} seconds", seconds);
        thread::sleep(Duration::from_secs(seconds));
    }
}
