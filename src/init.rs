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
    println!("{}", usage());
}

fn usage() -> String {
    r#"sweetch-bot - notify about game sales

USAGE:
    sweetch-bot [FLAGS]

FLAGS:
    --init      Initialize configuration
"#
    .to_string()
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
    cfg.write_all(init_config().as_bytes())?;
    Ok(())
}

fn init_config() -> String {
    r#"[schedule]
run_at = ["7:00 pm"]

[[watched_game]]
title = "Game 1 title here"

[[watched_game]]
title = "Game 2 title here"
"#
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_incorrect_args_passed_with_correct_args() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into(), "--init".into()];

        // when
        let incorrect_args_passed = incorrect_args_passed(&args);

        // then
        assert_eq!(incorrect_args_passed, false);
    }

    #[test]
    fn test_incorrect_args_passed_with_too_many_args() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into(), "--init".into(), "TOO_MANY".into()];

        // when
        let incorrect_args_passed = incorrect_args_passed(&args);

        // then
        assert_eq!(incorrect_args_passed, true);
    }

    #[test]
    fn test_incorrect_args_passed_without_init() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into(), "--other".into()];

        // when
        let incorrect_args_passed = incorrect_args_passed(&args);

        // then
        assert_eq!(incorrect_args_passed, true);
    }

    #[test]
    fn test_incorrect_args_passed_with_too_few_args() {
        // given
        let args: Vec<String> = vec!["NOT_IMPORTANT".into()];

        // when
        let incorrect_args_passed = incorrect_args_passed(&args);

        // then
        assert_eq!(incorrect_args_passed, false);
    }

    #[test]
    fn test_usage() {
        // when
        let usage = usage();

        // then
        assert!(usage.contains("USAGE"));
        assert!(usage.contains("--init"));
    }

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

    #[test]
    fn test_init_config() {
        // when
        let init_config = init_config();

        // then
        assert!(init_config.contains("[schedule]"));
        assert!(init_config.contains("run_at"));
        assert!(init_config.contains("[[watched_game]]"));
        assert!(init_config.contains("title"));
    }
}
