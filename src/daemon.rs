use anyhow::Result;

pub(crate) fn daemonize<F: FnOnce() -> Result<()>>(fun: F) -> Result<()> {
    fun()?;
    Ok(())
}
