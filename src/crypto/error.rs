use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidKeyLength,
    InvalidTextKeyPair
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidKeyLength => write!(f, "\x1b[1;31mInvalidKeyLength\x1b[0m: Key must have a minimum byte length of 4 and a maximum of 56."),
            Error::InvalidTextKeyPair => write!(f, "\x1b[1;31mInvalidTextKeyPair\x1b[0m: Provided text and key do not constitute a valid pair for encryption/decryption."),
        }
        
    }
}

impl error::Error for Error {}
