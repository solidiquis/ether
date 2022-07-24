use std::{env, path::PathBuf};

pub const HELP: &'static str = r#"
USAGE:
    tcrypt [mode] [options]

DESCRIPTION:
    Symmetric-key encryption and decryption utility.

EXAMPLE:
    tcrypt enc -t "Text to encrypt." -k foobar

ARGUMENTS:
    mode [enc|dec] Specify whether to encrypt or decrypt given text and key.

OPTIONS:
    -t             Input text to encrypt or decrypt.
    -k             Cryptographic key. Valid lengths: [4, 56] exclusive.
    -i             Path to file containing text to encrypt or decrypt.
    -p             Path to file containing cryptographic key.
"#;

pub struct TCryptArgs {
    pub mode: Option<Mode>,
    pub text: Option<String>,
    pub key: Option<String>,
    pub path_to_text: Option<PathBuf>,
    pub path_to_key: Option<PathBuf>,
}

pub enum Mode {
    Encipher,
    Decipher
}

impl Default for TCryptArgs {
    fn default() -> Self {
        Self {
            mode: None,
            text: None,
            key: None,
            path_to_text: None,
            path_to_key: None,
        }
    } 
}

impl TCryptArgs {
    pub fn init() -> Option<Self> {
        let mut tcrypt_args = Self::default();

        let clargs = env::args().collect::<Vec<String>>();

        tcrypt_args.mode = Self::parse_mode(&clargs[1]);

        let opts = &clargs[2..];

        if opts.len() == 0 { return None }

        for chunk in opts.chunks(2) {
            let arg = &chunk[0];
            let value = &chunk[1];

            if arg == "-t" {
                tcrypt_args.text = Some(value.to_owned());
            } else if arg == "-k" {
                tcrypt_args.key = Some(value.to_owned());
            } else if arg == "-i" {
                tcrypt_args.path_to_text = Some(PathBuf::from(value))
            } else if arg == "-p" {
                tcrypt_args.path_to_key = Some(PathBuf::from(value))
            } else if arg == "-h" {
                return None
            }
        }

        Some(tcrypt_args)
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
