use std::fs;

mod cli;
mod crypto;

fn main() -> Result<(), crypto::error::Error> {
    use cli::TCryptArgs;
    use cli::Mode;

    let args = match TCryptArgs::init() {
        Some(a) => a,
        None => {
            println!("{}", cli::HELP);
            return Ok(())
        }
    };

    let TCryptArgs {
        text,
        key,
        mode,
        path_to_key,
        path_to_text
    } = args;

    let crypto_mode = mode.expect("Missing argument `[enc|dec]`.");

    let crypto_key = key.or_else(|| {
        let key = path_to_key
            .map(|path| fs::read(path).expect("Failed to read key provided by path, `-p`."))
            .map(|bytes| String::from_utf8(bytes).expect("Failed to parse key file provided by `-p`."))
            .expect("Key `-k` or path to key `-p` required.");

        Some(key)
    }).expect("Failed to parse key.");

    let crypto_text = text.or_else(|| {
        let text = path_to_text
            .map(|path| fs::read(path).expect("Failed to read text provided by path, `-i`."))
            .map(|bytes| String::from_utf8(bytes).expect("Failed to parse text file provided by `-i`."))
            .expect("Text `-t` or path to text `-i` required.");

        Some(text)
    }).expect("Failed to parse text.");

    let result = match crypto_mode {
        Mode::Decipher => crypto::decipher(&crypto_text, &crypto_key),
        Mode::Encipher => crypto::encipher(&crypto_text, &crypto_key)
    }?;

    println!("{}", result);

    Ok(())
}
