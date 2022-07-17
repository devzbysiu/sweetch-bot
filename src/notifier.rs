use crate::switch::Game;

use anyhow::Result;
use log::info;
use rutils::desktop_notifier::notify;
use std::cmp;
use std::fmt::Write;

const MAX_GAMES_IN_NOTIFICATION: usize = 10;

pub(crate) fn notify_success(games: &[Game]) -> Result<()> {
    info!("found games on sale - sending notification");
    notify(&build_body(games)?)?;
    Ok(())
}

fn build_body(games: &[Game]) -> Result<String> {
    let max_len = cmp::min(MAX_GAMES_IN_NOTIFICATION, games.len());
    let mut body = String::new();
    for game in games.iter().take(max_len) {
        writeln!(body, "- {}", game.title())?;
    }
    if games.len() > MAX_GAMES_IN_NOTIFICATION {
        write!(body, "and {} more", games.len() - MAX_GAMES_IN_NOTIFICATION)?;
    }
    Ok(body)
}

pub(crate) fn notify_failure() -> Result<()> {
    info!("no games on sale found - sending notification");
    notify("No games on sale found.")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_body_with_few_games() {
        // given
        let games = vec![Game::new("Game 1"), Game::new("Game 2")];

        // when
        let body = build_body(&games).unwrap();

        // then
        assert_eq!(body, "- Game 1\n- Game 2\n");
    }

    #[test]
    fn test_build_body_with_too_many_games() {
        // given
        let games = vec![Game::new("Game Title"); MAX_GAMES_IN_NOTIFICATION + 1];
        let expected = format!("{}and 1 more", "- Game Title\n".repeat(10));

        // when
        let body = build_body(&games).unwrap();

        // then
        assert_eq!(body, expected);
    }
}
