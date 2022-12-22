#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources.
check:
    cargo clippy --locked -- -D warnings

# Builds and opens documentation in-browser without the dependencies docs.
doc:
    cargo doc --open --no-deps

# Builds and opens documentation in-browser with the dependencies docs.
docs-deps:
    cargo doc --open

# Restart docker
docker-restart:
    sudo systemctl restart docker

# Build docker.
docker-build:
    docker build -t photo-story:distroless -f Dockerfile .

# Docker compose up.
docker-compose-up:
    docker-compose up

# Docker compose down.
docker-compose-down:
    docker-compose down

# Cargo and clippy fix.
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Make .githooks this project hooks lookup directory.
init-git-hooks:
    git config --local core.hooksPath .githooks

# Install Loki Docker Driver plugin
install-loki-docker-driver:
    docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions
    sudo cp ./monitoring/loki/daemon.json /etc/docker/daemon.json
    sudo systemctl restart docker

# Install mold linker for faster builds.
install-mold-linker:
    git clone https://github.com/rui314/mold.git
    mkdir mold/build
    cd mold/build
    git checkout v1.7.1
    ../install-build-deps.sh
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ ..
    cmake --build . -j $(nproc)
    sudo cmake --install .
    cd ../..
    rm -rf mold

# Format using custom rustfmt.
rustfmt:
    find -type f -path "./crates/*" -path "*.rs" | xargs ./rustfmt --edition 2021

# Runs all tests.
test-all:
    cargo test --locked

# Runs tests of the specified package.
test PACKAGE:
    cargo test -p {{PACKAGE}} --locked

# Install cargo udeps.
udeps-install:
    cargo install cargo-udeps --locked

# Use udeps to find unused dependencies.
udeps:
    cargo +nightly udeps

# Vendor all dependencies locally.
vendor:
    cargo vendor
