# Contributing to `bendecode`

`Bendecode` is a small bencode parser for rust which chelps collect the data
from a bencode file into a rust acceptable format (structs and enums).

## Styling and formatting

Code is formatted using rustfmt and linted using clippy
You can install these with

```sh
rustup component add rustfmt
rustup component add clippy
```

Make sure you have [rust](https://www.rust-lang.org/tools/install) installed

Make sure you've given the necessary permissions to the pre-commit hooks

```sh
chmod 500 ./scripts/setup-hooks.sh ./.hooks/pre-commit.sh
```

Make sure to check for typos (preferably using [typos](https://github.com/crate-ci/typos))

Thank you for contributing
