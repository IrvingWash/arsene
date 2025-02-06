# arsene

![Crates.io Version](https://img.shields.io/crates/v/arsene)

A simple CLI tool to download albums from bandcamp. Only 128 kbps.

# Contents
- [Installation](#installation)<br>
- [Command-line options](#command-line-options)<br>
- [Usage](#usage)<br>
- [Additional links](#additional-links)<br>

## Installation
### From crates.io
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Run in your ternimal:
```
cargo install arsene
```

### Manual
1. Clone the repo
2. `cargo install --path /path/to/the/repo`

## Command-line options
-a, --album-url <ALBUM_URL>  # Album URL on bandcamp <br>
-s, --save-path <SAVE_PATH>  # The path where the downloaded files will be saved <br>
-h, --help                   # Print help <br>
-V, --version                # Print version <br>

## Usage
```bash
arsene --album-url <ALBUM_URL> --save-path <SAVE_PATH>
```

## Additional links
[Check out my bandcamp album path fixer](https://github.com/IrvingWash/bc_unshit)
