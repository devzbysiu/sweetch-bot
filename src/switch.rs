use anyhow::Result;
use log::info;

pub(crate) fn games_on_sale() -> Result<Vec<Game>> {
    info!("checking games on sale");
    Ok(vec![
        Game {
            name: "some".into(),
        },
        Game {
            name: "game".into(),
        },
        Game {
            name: "game".into(),
        },
        Game {
            name: "game".into(),
        },
        Game {
            name: "game".into(),
        },
        Game {
            name: "game".into(),
        },
    ])
}

pub(crate) struct Game {
    pub(crate) name: String,
}
