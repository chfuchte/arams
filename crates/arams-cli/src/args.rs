use crate::constants::{
    BIN_NAME, BUILD_TIMESTAMP_UTC, LAST_COMMIT_DATE, LAST_COMMIT_ID, LAST_COMMIT_ID_LONG, VERSION,
};
use clap::{ArgAction, ArgGroup, arg, builder::ValueParser, command, value_parser};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Args {
    input: Input,
}

impl Args {
    pub(crate) fn new(input: Input) -> Self {
        Self { input }
    }

    pub(crate) fn input(&self) -> &Input {
        &self.input
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Input {
    File(PathBuf),
    Raw(String),
    None,
}

pub(crate) fn parse_args_or_exit() -> Result<Args, clap::Error> {
    let command = command!()
        .name(BIN_NAME)
        .about(None)
        // help
        .next_line_help(false)
        // version
        .disable_version_flag(true)
        .arg(
            arg!(-v --version "Print version")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool)),
        )
        // license
        .arg(
            arg!(--license "Print license")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool)),
        )
        // build info
        .arg(
            arg!(--"build-info" "Print build information" )
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool)),
        )
        .group(ArgGroup::new("readonly-info-args").multiple(false).args([
            "version",
            "license",
            "build-info",
        ]))
        // input file
        .arg(
            arg!([INPUT] "Input file path, raw string, or omit to read from stdin")
                .action(ArgAction::Set)
                .value_parser(ValueParser::new(parse_input)),
        )
        .group(ArgGroup::new("run-args").multiple(true).args(["INPUT"]));

    let matches = command.get_matches();

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

    let input = matches
        .get_one::<Input>("INPUT")
        .cloned()
        .unwrap_or(Input::None);

    Ok(Args::new(input))
}

fn parse_input(input_str: &str) -> Result<Input, clap::Error> {
    let path = Path::new(input_str);
    if path.exists() {
        Ok(Input::File(path.canonicalize().map_err(|e| {
            clap::Error::raw(clap::error::ErrorKind::Io, e)
        })?))
    } else {
        Ok(Input::Raw(input_str.to_string()))
    }
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
    if let Some(commit_id) = LAST_COMMIT_ID_LONG {
        println!("build from commit: {}", commit_id);
    }
    if let Some(build_timestamp) = BUILD_TIMESTAMP_UTC {
        println!("build at {} UTC", build_timestamp);
    }
}
