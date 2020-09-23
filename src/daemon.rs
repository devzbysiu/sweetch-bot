use anyhow::Result;
use daemonize::Daemonize;
use std::fs::File;

pub(crate) fn daemonize<F: FnOnce() -> Result<()>>(fun: F) -> Result<()> {
    let daemonize = Daemonize::new()
        .stdout(File::create("/tmp/sweetch-bot.stdout.log")?)
        .stderr(File::create("/tmp/sweetch-bot.stderr.log")?);

    match daemonize.start() {
        Ok(_) => fun()?,
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
}
