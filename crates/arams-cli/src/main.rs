use arams_core::{IntoSourceCode, compile, execute};

use crate::{
    args::{Input, parse_args_or_exit},
    errors::{Error, StatusCode},
};

mod args;
mod constants;
mod errors;
mod fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match run() {
        Ok(_) => std::process::exit(StatusCode::Success.into()),
        Err(e) => {
            eprintln!("{}", e.standalone_message());
            std::process::exit(e.status_code().into());
        }
    }
}

fn run() -> Result<(), Error> {
    let args = parse_args_or_exit().map_err(Error::FailedToParseArgs)?;

    let contents = match args.input() {
        Input::File(file_path) => fs::read_file(file_path)?,
        Input::Raw(raw_string) => raw_string.into_lines(),
        Input::None => fs::read_stdin()?,
    };

    let registers = args.registers();

    let program = compile(contents).map_err(Error::ARAMSFailedToCompile)?;
    let machine = execute(program, Some(registers.clone())).map_err(Error::ARAMSFailedToExecute)?;

    println!("Machine State After Execution:");
    println!("Accumulator: {}", machine.get_accumulator());
    println!("Registers:");
    for (name, value) in machine.get_registers().iter() {
        println!("  {}: {}", name, value);
    }

    Ok(())
}
