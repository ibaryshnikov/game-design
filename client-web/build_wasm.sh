cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir dist/pkg target/wasm32-unknown-unknown/debug/client_web.wasm
