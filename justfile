#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources.
check:
    cargo clippy --locked -- -D warnings

# Cargo and clippy fix.
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Format using custom rustfmt.
rustfmt:
    find -type f -not -path "*/[@.]*" -not -path "*/target/*" -not -path "*/vendor/*" -path "*.rs" | xargs ./rustfmt

# Vendor all dependencies locally.
vendor:
    cargo vendor

# Build docker.
docker-build:
    docker build -t photo-story:distroless -f Dockerfile .

# Run docker.
docker-run:
    docker run -ti photo-story:distroless ./monitoring

# Docker compose up.
docker-compose-up:
    docker-compose up

# Docker compose down.
docker-compose-down:
    docker-compose down

# Runs all tests.
test-all:
    cargo test --locked

# Runs tests of the specified package.
test PACKAGE:
    cargo test -p {{PACKAGE}} --locked
