use std::env;
use std::process::ExitCode;

mod cli;
mod crypto;

#[cfg(test)]
mod test;

fn run(clargs: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use cli::{Mode, EtherArgs, error::Error as CLError};

    let EtherArgs {
        text,
        key,
        mode,
        path_to_key,
        path_to_text,
        help
    } = EtherArgs::init(clargs)?;

    if help {
        println!("{}", cli::HELP);
        return Ok(())
    }

    let crypto_mode = mode.ok_or(CLError::ArgumentError)?;

    let read_to_string = |path| fs::read_to_string(path).ok();

    let crypto_key = key
        .or_else(|| path_to_key.and_then(read_to_string))
        .ok_or(CLError::MissingKeyError)?;

    let crypto_text = text
        .or_else(|| path_to_text.and_then(read_to_string))
        .ok_or(CLError::MissingInputError)?;

    let result = match crypto_mode {
        Mode::Decipher => crypto::decipher(crypto_text.trim_end(), crypto_key.trim_end()),
        Mode::Encipher => crypto::encipher(crypto_text.trim_end(), crypto_key.trim_end())
    }?;

    println!("{}", result);

    Ok(())
}

fn main() -> ExitCode {
    let clargs = env::args().collect::<Vec<String>>();

    if let Err(e) = run(clargs) {
        eprintln!("{}", e.to_string());
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
