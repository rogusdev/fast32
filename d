#!/bin/fish
cargo doc --all-features
caddy file-server --root ./target/doc --listen :5000
