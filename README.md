# dio

## Setup Instructions

* macOS
  1. Install Rust
     1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  2. Install homebrew
     1. `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
  3. Install node.js
     1. `brew install node`
  4. Install wasm-pack with Cargo (Rust's package manager/build tool/packaging and distribution tool. It's amazing.)
     1. `cargo install wasm-pack`
  5. Install the node dependencies for the repo inside of the `dio` folder
     1. `npm install`
* Windows
  1. TODO:

## Development

1. Run `npm run serve` to start the web server. This also compiles your Rust. The first time you execute this should take a long time (minutes)
2. Open `localhost:8080` in your favorite browser (except if IE is your favorite. Feel free to leave the project if it is.)
