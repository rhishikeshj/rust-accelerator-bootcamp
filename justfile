alias cl := clean
alias cp := clippy
alias c  := check
alias f  := fmt

alias t  := test

alias bapp := build-app
alias bauth := build-auth
alias b  := build
alias br  := build-release

default:
    just --list

clean:
    cargo clean
    
fmt:
    cargo fmt --check

build-auth:
    cargo build --package auth-service

build-app:
    cargo build --package app-service

build-auth-release:
    cargo build --release --package auth-service

build-app-release:
    cargo build --release --package app-service

rebuild: clean build-auth

build: build-auth build-app

build-release: build-auth-release build-app-release

clippy:
    cargo clippy --locked --all-targets -- -D warnings

test:
    cargo nextest run --locked

check: fmt clippy test



