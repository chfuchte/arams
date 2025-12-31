use crate::args::parse_args_or_exit;

mod args;
mod constants;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = parse_args_or_exit()?;

    Ok(())
}
