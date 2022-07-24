# TCrypt

```
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
```

## Installation

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install --git https://github.com/solidiquis/tcrypt`
3. The executable should then be located in `$HOME/.cargo/bin/`.
