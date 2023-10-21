#!/bin/bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx database setup
cargo build --release