pub mod error;

use std::path::PathBuf;

pub const HELP: &'static str = r#"
USAGE:
    ether ARGUMENT OPTIONS

DESCRIPTION:
    Symmetric-key encryption and decryption utility built on Blowfish.

EXAMPLE:
    ether enc -i "Text to encrypt." -k foobar

ARGUMENT:
    enc | dec      Specify whether to encrypt or decrypt given text and key.

OPTIONS:
    -i             Input text to encrypt or decrypt.
    -I             Path to file containing text to encrypt or decrypt.
    -k             Cryptographic key to encrypt or decrypt text. Valid lengths: [4, 56] exclusive.
    -K             Path to file containing cryptographic key. Valid lengths: [4, 56] exclusive.
"#;

pub struct EtherArgs {
    pub mode: Option<Mode>,
    pub text: Option<String>,
    pub key: Option<String>,
    pub path_to_text: Option<PathBuf>,
    pub path_to_key: Option<PathBuf>,
    pub help: bool
}

#[derive(Debug)]
pub enum Mode {
    Encipher,
    Decipher
}

impl Default for EtherArgs {
    fn default() -> Self {
        Self {
            mode: None,
            text: None,
            key: None,
            path_to_text: None,
            path_to_key: None,
            help: false,
        }
    } 
}

impl EtherArgs {
    pub fn init(clargs: Vec<String>) -> Result<Self, error::Error> {
        let mut ether_args = Self::default();

        if clargs.len() == 1 { return Err(error::Error::UsageError) }

        if clargs.contains(&"-h".to_string()) {
            ether_args.help = true;
            return Ok(ether_args)
        }

        let mode_arg = clargs.get(1).ok_or(error::Error::UsageError)?;

        if let Some(mode) = Self::parse_mode(&mode_arg) {
            ether_args.mode = Some(mode);
        } else {
            return Err(error::Error::ArgumentError);
        }

        let opts = &clargs[2..];

        for chunk in opts.chunks(2) {
            let arg = chunk.get(0).ok_or(error::Error::UsageError)?;
            let value = chunk.get(1).ok_or(error::Error::UsageError)?;

            if arg == "-i" {
                ether_args.text = Some(value.to_owned());
            } else if arg == "-k" {
                ether_args.key = Some(value.to_owned());
            } else if arg == "-I" {
                ether_args.path_to_text = Some(PathBuf::from(value))
            } else if arg == "-K" {
                ether_args.path_to_key = Some(PathBuf::from(value))
            }
        }

        Ok(ether_args)
    }

    fn parse_mode(value: &str) -> Option<Mode> {
        if value == "enc" {
            Some(Mode::Encipher)
        } else if value == "dec" {
            Some(Mode::Decipher)
        } else {
            None
        }
    }
}
