set windows-shell := ["powershell.exe"]

fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings --no-deps

check: fmt clippy

unit-test:
    cargo test --bins

test: unit-test