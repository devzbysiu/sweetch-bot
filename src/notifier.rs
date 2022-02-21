use crate::switch::Game;
use anyhow::Result;
use log::info;
use notify_rust::{Notification, Timeout, Urgency};
use std::cmp;

const MAX_GAMES_IN_NOTIFICATION: usize = 10;

pub(crate) fn notify_success(games: &[Game]) -> Result<()> {
    info!("found games on sale - sending notification");
    Notification::new()
        .summary("Game Available")
        .body(&build_body(games))
        .timeout(Timeout::Never)
        .urgency(Urgency::Critical)
        .show()?;
    Ok(())
}

fn build_body(games: &[Game]) -> String {
    let max_len = cmp::min(MAX_GAMES_IN_NOTIFICATION, games.len());
    let mut body = String::new();
    for game in games.iter().take(max_len) {
        body.push_str(&format!("- {}\n", game.title()));
    }
    if games.len() > MAX_GAMES_IN_NOTIFICATION {
        body.push_str(&format!(
            "and {} more",
            games.len() - MAX_GAMES_IN_NOTIFICATION
        ));
    }
    body
}

pub(crate) fn notify_failure() -> Result<()> {
    info!("no games on sale found - sending notification");
    Notification::new()
        .summary("No Gamges")
        .body("No games on sale found.")
        .timeout(Timeout::from(10_000))
        .urgency(Urgency::Low)
        .show()?;
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
        let body = build_body(&games);

        // then
        assert_eq!(body, "- Game 1\n- Game 2\n");
    }

    #[test]
    fn test_build_body_with_too_many_games() {
        // given
        let games = vec![Game::new("Game Title"); MAX_GAMES_IN_NOTIFICATION + 1];
        let expected = format!("{}and 1 more", "- Game Title\n".repeat(10));

        // when
        let body = build_body(&games);

        // then
        assert_eq!(body, expected);
    }
}
