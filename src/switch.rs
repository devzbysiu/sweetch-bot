use anyhow::Result;
use log::info;

pub(crate) fn are_games_on_sale() -> Result<bool> {
    info!("checking games on sale");
    Ok(false)
}
