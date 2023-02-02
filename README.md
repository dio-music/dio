# dio

## Setup Instructions

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install node.js](https://nodejs.org/en/download/)
3. Install wasm-pack with Cargo (Rust's package manager/build tool/packaging and distribution tool. It's amazing.)
   1. `cargo install wasm-pack`
4. Install the node dependencies for the repo inside of the `dio` folder
   1. `npm install`

## Development

1. Run `npm run serve` to start the web server. This also compiles your Rust code as a web-assembly package. The first time you execute this should take a long time (minutes)
2. Open `localhost:8080` in your favorite browser (except if IE is your favorite. Feel free to leave the project if it is.)
