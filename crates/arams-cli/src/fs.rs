use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal};

use crate::errors::Error;

pub(crate) fn read_file(file_path: &std::path::Path) -> Result<Vec<String>, Error> {
    let file = File::open(file_path).map_err(Error::IOFailedToReadFile)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .map_err(Error::IOFailedToReadFile)?;

    Ok(lines)
}

pub(crate) fn read_stdin() -> Result<Vec<String>, Error> {
    let stdin = std::io::stdin();

    if stdin.is_terminal() {
        return Err(Error::NoInput);
    }

    let reader = BufReader::new(stdin.lock());
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .map_err(Error::IOFailedToReadFromStdIn)?;

    Ok(lines)
}
