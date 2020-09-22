use anyhow::Result;

pub(crate) fn schedule<F: FnOnce() -> Result<()>>(fun: F) -> Result<()> {
    fun()?;
    Ok(())
}
