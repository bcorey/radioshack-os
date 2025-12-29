build:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-name packed \
        --out-dir stage/target \
        --target web target/wasm32-unknown-unknown/release/tv-gui-3d.wasm

serve:
    just build
    basic-http-server stage
