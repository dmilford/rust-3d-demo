# rust-3d-demo
  An example of how you can do 3D graphics in the browser with Rust and WebGL.

# Prerequisites
  Rustup: 
    I'm using version 1.19.0.  If you have trouble, you might want to peg your verions to that.
    To check what version you have, you can type 'rustup --version' in a command line terminal.
    If you need help getting this set up, I have a youtube tutorial at https://youtu.be/f6tizikEMTk
  Visual Studio Code: 
    To install, go to https://code.visualstudio.com/
    This is the dev environment I use, but of course you can use your favorite.
    If you need help getting this environment set up, I have a youtube tutorial at https://youtu.be/f6tizikEMTk

# Initialization
    In your command line terminal, type...
        rustup target add wasm32-unknown-unknown
            - This will make sure you have wasm as a target for Rust
        cargo build --target wasm32-unknown-unknown
            - This creates a wasm build in ./target
        wasm-bindgen --out-dir . --target web --reference-types --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/debug/rust_3d_demo.wasm
            - This will write out the JavaScript web binding to the current directlry

# Running
    Serve the repo root with your web server and navigate to `/` in your web browser.
