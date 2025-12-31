use clap::{ArgAction, arg, command, value_parser};

use crate::constants::{
    AFTER_HELP_TEXT, BIN_NAME, BUILD_TIMESTAMP_UTC, LAST_COMMIT_DATE, LAST_COMMIT_ID,
    LAST_COMMIT_ID_LONG, PROJECT_DESCRIPTION, VERSION,
};

pub(crate) struct Args {}

impl Args {}

pub(crate) fn parse_args_or_exit() -> Result<Args, Box<dyn std::error::Error>> {
    let matches = command!()
        .name(BIN_NAME)
        .about(PROJECT_DESCRIPTION)
        // help
        .after_help(AFTER_HELP_TEXT)
        .after_long_help(AFTER_HELP_TEXT)
        .next_line_help(true)
        // version
        .disable_version_flag(true)
        .arg(
            arg!(-v --version "Print version")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // license
        .arg(
            arg!(--license "Print license")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        // build info
        .arg(
            arg!(--"build-info" "Print build information" )
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("license") {
        print_license();
        std::process::exit(0);
    } else if matches.get_flag("version") {
        print_version();
        std::process::exit(0);
    } else if matches.get_flag("build-info") {
        print_build_info();
        std::process::exit(0);
    }

    Ok(Args {})
}

fn print_license() {
    println!(include_str!("../../../LICENSE.txt"));
}

fn print_version() {
    match (LAST_COMMIT_ID, LAST_COMMIT_DATE) {
        (Some(commit_id), Some(commit_date)) => {
            println!("{} {} ({} {})", BIN_NAME, VERSION, commit_id, commit_date,);
        }
        _ => {
            println!("{} {}", BIN_NAME, VERSION);
        }
    }
}

fn print_build_info() {
    print_version();
    match (LAST_COMMIT_ID_LONG, BUILD_TIMESTAMP_UTC) {
        (Some(commit_id), Some(build_timestamp)) => {
            println!("build from commit: {}", commit_id);
            println!("build at {} UTC", build_timestamp);
        }
        _ => {}
    }
}
