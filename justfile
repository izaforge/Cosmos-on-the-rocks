set shell := ["powershell", "-NoProfile", "-Command"]

watch:
    cargo watch -cx run

watch-web:
    cargo watch -cx "run --target wasm32-unknown-unknown"