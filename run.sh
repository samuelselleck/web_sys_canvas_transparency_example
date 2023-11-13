RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target=web
cd pkg
python3 -m http.server