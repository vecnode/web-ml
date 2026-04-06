# texture-bevy-egui-app

A small Bevy + Egui desktop app template for texture workspace UI experiments, with WASM compilation support.

### Dependencies

```sh
# One-time setup
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli --version 0.2.114
```

### Reproduce

```sh
# Development
cargo run

# Release Desktop
cargo build --release

# Release WASM
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir web/pkg --out-name texture-bevy-egui-app target/wasm32-unknown-unknown/release/texture-bevy-egui-app.wasm
python3 -m http.server 8080
```

Open: http://localhost:8080/web/

### License

MIT

