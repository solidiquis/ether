# Ether

```
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
```

## Installation

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install --git https://github.com/solidiquis/ether`
3. The executable should then be located in `$HOME/.cargo/bin/`.
