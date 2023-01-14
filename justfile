#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Build backend and frontend for release.
build-release:
    mkdir -p ./photo-story
    mkdir -p ./photo-story/static
    mkdir -p ./photo-story/logs
    cargo build --release --bin backend
    trunk build --release ./crates/frontend/index.html --dist ./photo-story/static --public-url /static/
    mv ./target/release/backend ./photo-story/backend
    cp -r ./assets ./photo-story
    cp -r ./configs ./photo-story

# Cleans the project.
clean:
    rm -rf ./photo-story
    cargo clean
    trunk clean

# Runs clippy on the sources.
check:
    cargo clippy --locked -- -D warnings

# Builds and opens documentation in-browser without the dependencies docs.
docs:
    cargo doc --open --no-deps

# Builds and opens documentation in-browser with the dependencies docs.
docs-deps:
    cargo doc --open

# Restart docker service.
docker-restart:
    sudo systemctl restart docker

# Build project docker container.
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

# Install frontend needed dependencies.
install-frontend-deps:
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk

# Install Loki Docker Driver plugin to monitor containers.
install-loki-docker-driver:
    docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions
    sudo cp ./monitoring/loki/daemon.json /etc/docker/daemon.json
    sudo systemctl restart docker

# Install mold linker for faster builds.
install-mold-linker:
    rm -rf mold
    git clone https://github.com/rui314/mold.git
    mkdir ./mold/build
    sudo ./mold/install-build-deps.sh
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ ./mold/ -B ./mold/build
    cmake --build ./mold/build/ -j $(nproc)
    sudo cmake --install ./mold/build/
    rm -rf mold

# Install cargo udeps.
install-udeps:
    cargo install cargo-udeps --locked

# Run backend in dev environment.
run-backend PORT:
    GENERAL_DEFAULT_RUN_ENV=development cargo run --bin backend -- --port {{PORT}}

# Run backend in prod environment.
run-backend-prod PORT:
    GENERAL_DEFAULT_RUN_ENV=production cargo run --bin backend --release -- --port {{PORT}}

# Run both backend and frontend in dev with watch.
run-dev BACKEND_PORT FRONTEND_PORT:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-serve {{BACKEND_PORT}} {{FRONTEND_PORT}}' & \
    bash -c 'cargo watch -- just run-backend {{BACKEND_PORT}}')

# Run both backend and frontend in prod with watch.
run-prod BACKEND_PORT FRONTEND_PORT:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-serve-prod {{BACKEND_PORT}} {{FRONTEND_PORT}}' & \
    bash -c 'cargo watch -- just run-backend-prod {{BACKEND_PORT}}')

# Format using custom rustfmt.
rustfmt:
    find -type f -path "./crates/*" -path "*.rs" | xargs ./rustfmt --edition 2021

# Serve frontend in a development runtime enviroment.
trunk-serve BACKEND_PORT PORT:
    BACKEND_ADDR=$(just _grep_toml_config ./configs/backend/server.toml default addr) \
    && GENERAL_DEFAULT_RUN_ENV=development trunk serve ./crates/frontend/index.html --address 127.0.0.1 --port {{PORT}} --proxy-backend=http://$BACKEND_ADDR:{{BACKEND_PORT}}/api/

# Serve frontend in a production runtime enviroment.
trunk-serve-prod BACKEND_PORT PORT:
    BACKEND_ADDR=$(just _grep_toml_config ./configs/backend/server.toml production addr) \
    && GENERAL_DEFAULT_RUN_ENV=production trunk serve --release ./crates/frontend/index.html --address 127.0.0.1 --port {{PORT}} --proxy-backend=http://$BACKEND_ADDR:{{BACKEND_PORT}}/api/

# Runs all tests.
test-all:
    cargo test --locked

# Runs tests of the specified package.
test PACKAGE:
    cargo test -p {{PACKAGE}} --locked

# Use udeps to find unused dependencies.
udeps:
    cargo +nightly udeps

# Vendor all dependencies locally.
vendor:
    cargo vendor

_grep_toml_config FILE GROUP_ENV CONFIG_VAR:
    grep -A 100 "^\[{{GROUP_ENV}}\]" {{FILE}} | grep -m 1 -oP '^{{CONFIG_VAR}}\s?=\s?"?\K[^"?]+'
