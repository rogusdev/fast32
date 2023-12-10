#!/bin/fish
cargo doc
caddy file-server --root ./target/doc --listen :5000
