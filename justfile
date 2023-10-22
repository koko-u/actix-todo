_default:
    @just --list

run:
    cargo run

watch:
    cargo watch --quiet --clear --exec run

build:
    cargo watch --quiet --clear --exec build

test:
    cargo watch --quiet --clear --exec "nextest run"