use crate::constants::{
    BIN_NAME, BUILD_TIMESTAMP_UTC, LAST_COMMIT_DATE, LAST_COMMIT_ID, LAST_COMMIT_ID_LONG, VERSION,
};
use clap::{ArgAction, ArgGroup, arg, builder::ValueParser, command, value_parser};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub(crate) struct Args {
    input: Input,
    registers: HashMap<usize, u64>,
}

impl Args {
    pub(crate) fn new(input: Input, registers: HashMap<usize, u64>) -> Self {
        Self { input, registers }
    }

    pub(crate) fn input(&self) -> &Input {
        &self.input
    }

    pub(crate) fn registers(&self) -> &HashMap<usize, u64> {
        &self.registers
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
        .arg(arg!(-r --registers <VALUES> "Preseed the registers of the simulated machine (format: [(1,2),(2,4)], default: all registers at 0)").action(ArgAction::Set).value_parser(parse_register_preseed))
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

    let registers = matches
        .get_one::<HashMap<usize, u64>>("registers")
        .cloned()
        .unwrap_or_else(HashMap::new);

    Ok(Args::new(input, registers))
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

fn parse_register_preseed(input_str: &str) -> Result<HashMap<usize, u64>, clap::Error> {
    let trimmed = input_str.trim();
    if trimmed.is_empty() {
        return Ok(HashMap::new());
    }

    let pairs: Vec<&str> = trimmed
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split("),(")
        .collect();

    let mut register_map = HashMap::new();

    for pair in pairs {
        let clean_pair = pair.trim_matches(|c| c == '(' || c == ')');
        let nums: Vec<&str> = clean_pair.split(',').collect();
        if nums.len() != 2 {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Invalid register pair: {}", pair),
            ));
        }

        let reg_index: usize = nums[0].trim().parse().map_err(|e| {
            clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Invalid register index in pair {}: {}", pair, e),
            )
        })?;

        let reg_value: u64 = nums[1].trim().parse().map_err(|e| {
            clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Invalid register value in pair {}: {}", pair, e),
            )
        })?;

        register_map.insert(reg_index, reg_value);
    }

    Ok(register_map)
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
