use crate::constants::{
    AFTER_HELP_TEXT, BIN_NAME, BUILD_TIMESTAMP_UTC, LAST_COMMIT_DATE, LAST_COMMIT_ID,
    LAST_COMMIT_ID_LONG, PROJECT_DESCRIPTION, VERSION,
};
use clap::{ArgGroup, arg, command, value_parser};
use std::path::PathBuf;

pub(crate) struct Args {
    file_path: PathBuf,
}

impl Args {
    pub(crate) fn new(file_path: PathBuf) -> Self {
        assert!(file_path.is_absolute(), "file_path must be absolute");
        Self { file_path }
    }

    pub(crate) fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

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
        .arg(arg!(-v --version "Print version").value_parser(value_parser!(bool)))
        // license
        .arg(arg!(--license "Print license").value_parser(value_parser!(bool)))
        // build info
        .arg(arg!(--"build-info" "Print build information" ).value_parser(value_parser!(bool)))
        .group(ArgGroup::new("readonly-info-args").multiple(false).args([
            "version",
            "license",
            "build-info",
        ]))
        // input file
        .arg(
            arg!([FILE] "Input file to process")
                .value_parser(value_parser!(PathBuf))
                .required_unless_present("readonly-info-args"),
        )
        .group(ArgGroup::new("run-args").multiple(true).args(["FILE"]))
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

    let file_path = matches
        .get_one::<PathBuf>("FILE")
        .expect("<FILE> is a required argument")
        .to_owned()
        .canonicalize()
        .expect("Failed to canonicalize file path");

    Ok(Args::new(file_path))
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
