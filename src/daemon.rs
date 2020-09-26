use anyhow::Result;
use daemonize::Daemonize;
use std::fs::File;
use std::path::{Path, PathBuf};

pub(crate) fn daemonize<F: FnOnce() -> Result<()>>(fun: F) -> Result<()> {
    let daemonize = Daemonize::new()
        .stdout(File::create(log_path("sweetch-bot.out"))?)
        .stderr(File::create(log_path("sweetch-bot.log"))?);

    match daemonize.start() {
        Ok(_) => fun()?,
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
}

fn log_path(filename: &str) -> PathBuf {
    Path::new(&std::env::temp_dir()).join(filename)
}
