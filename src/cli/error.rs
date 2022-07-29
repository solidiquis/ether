use std::fmt;

#[derive(Debug)]
pub enum Error {
    ArgumentError,
    UsageError,
    MissingKeyError,
    MissingInputError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ArgumentError => write!(f, "\x1b[1;31mArgumentError\x1b[0m: First argument must be either `enc` or `dec`."),
            Error::UsageError => write!(f, "\x1b[1;31mUsageError\x1b[0m: Missing or invalid arguments supplied. Refer to help text with `-h` option."),
            Error::MissingKeyError => write!(f, "\x1b[1;31mMissingKeyError\x1b[0m: Option `-k` or `-K` must be provided."),
            Error::MissingInputError => write!(f, "\x1b[1;31mMissingInputError\x1b[0m: Option `-i` or `-I` must be provided."),
        }
    }
}

impl std::error::Error for Error {}
