# Setup
Reference: [No Boilerplate](https://www.youtube.com/watch?v=ifaLk5v3W90)
## Install Rustup
It is an installer for Rust programming language and for Rust toolchain. This installs tools such as:
- `rustc` - Compiler
- `cargo` - Build Tool
- `clippy` - Linter
- `rust-format` - Auto Formatter
- Documentation tools, etc.

Go to the [Official Rustup Site](https://rustup.rs/). Copy the command and run it in your terminal. The command should be something like:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install AstroVim
- Install Neovim - `sudo pacman -S neovim`
- Backup everything - `mv ~/.config/nvim ~/.config/nvim.bak`
- Install AstroVim - `git clone --depth 1 https://github.com/AstroNvim/AstroNvim ~/.config/nvim`
- Run nvim

## Install Neovide
Add more things to NeoVim to make it more like an IDE.
```shell
git clone https://github.com/neovide/neovide
cd neovide
cargo build --release
```

## Install Test Runner - Bacon
```shell
cargo install bacon
```

## Install Essential Libraries
- tokio - Async runtime
- color-eyre - Error handling
- tracing - concurrent logging

## Useful libraries
- reqwuest - HTTP Requests
- rayon- concurrency and data parallelism
- aws-sdk-rust - Rust for AWS
- clap - command line args passing
- sqlx - compiled check SQL
- chrono - date-time aware
- yew.rs - Frontend
- clippy - Use at least these `unwrap_used` and `expect_used`
# Resources
- [No Boilerplate](https://github.com/0atman/noboilerplate)