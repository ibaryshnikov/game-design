cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir dist/pkg target/wasm32-unknown-unknown/release/client_web.wasm
