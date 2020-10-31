use anyhow::Result;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

pub(crate) fn handle_args(args: &[String]) -> Result<()> {
    if incorrect_args_passed(args) {
        print_usage();
        process::exit(1);
    }
    if init_arg_passed(args) {
        init()?;
        process::exit(0);
    }
    Ok(())
}

fn incorrect_args_passed(args: &[String]) -> bool {
    args.len() > 2 || (args.len() == 2 && args[1] != "--init")
}

fn print_usage() {
    println!(
        r#"sweetch-bot - notify about game sales

USAGE:
    sweetch-bot [FLAGS]

FLAGS:
    --init      Initialize configuration
"#
    )
}

fn init_arg_passed(args: &[String]) -> bool {
    args.len() >= 2 && args[1] == "--init"
}

fn init() -> Result<()> {
    create_dir_all(sweetch_dir()?)?;
    create_init_config()?;
    Ok(())
}

pub(crate) fn sweetch_dir() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .expect("failed to read config dir while init")
        .join("sweetch-bot"))
}

fn create_init_config() -> Result<()> {
    let mut cfg = File::create(sweetch_dir()?.join("sweetch-bot.toml"))?;
    cfg.write_all(
        r#"[schedule]
run_at = ["7:00 pm"]

[[watched_game]]
title = "Game 1 title here"

[[watched_game]]
title = "Game 2 title here"
"#
        .as_bytes(),
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_arg_passed_with_more_than_2_args() {
        // given
        let args: Vec<String> = vec![
            "NOT_IMPORTANT".into(),
            "--init".into(),
            "NOT_IMPORTANT".into(),
        ];

        // when
        let init_arg_passed = init_arg_passed(&args);

        // then
        assert_eq!(init_arg_passed, true);
    }

    #[test]
    fn test_init_arg_passed_when_not_passed() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into(), "--not-init".into()];

        // when
        let init_arg_passed = init_arg_passed(&args);

        // then
        assert_eq!(init_arg_passed, false);
    }

    #[test]
    fn test_init_arg_passed_with_exactly_2_args() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into(), "--init".into()];

        // when
        let init_arg_passed = init_arg_passed(&args);

        // then
        assert_eq!(init_arg_passed, true);
    }
}
