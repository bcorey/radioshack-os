build:
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --out-name packed \
        --out-dir stage/target \
        --target web target/wasm32-unknown-unknown/wasm-release/radioshack-os.wasm

serve:
    just build
    basic-http-server stage
