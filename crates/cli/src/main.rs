use crate::args::parse_args_or_exit;

mod args;
mod constants;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args_or_exit()?;

    println!("Using file: {}", args.file_path().display());

    Ok(())
}
