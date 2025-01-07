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
-a, --album-url <ALBUM_URL>  # Album URL on bandcamp
-s, --save-path <SAVE_PATH>  # The path where the downloaded files will be saved
-h, --help                   # Print help
-V, --version                # Print version

## Usage
```bash
arsene --album-url <ALBUM_URL> --save-path <SAVE_PATH>
```

## Additional links
[Check out my bandcamp album path fixer](https://github.com/IrvingWash/bc_unshit)
