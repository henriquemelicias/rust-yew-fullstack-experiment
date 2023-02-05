#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Build backend and frontend for release.
build-release:
    # Create new final directory.
    rm -rf ./photo-story
    mkdir -p ./photo-story/static ./photo-story/logs

    # Build backend and static files.
    cargo build --profile backend-release --bin backend
    trunk build --release --features ssr ./crates/frontend/index.html --dist ./photo-story/static --public-url /static/

    # Remove assets directory on the static folder (used when using CSR).
    rm -r ./photo-story/static/assets

    # Copy necessary files to final directory.
    cp -r ./assets ./photo-story
    cp -r ./configs ./photo-story
    cp -f ./target/backend-release/backend ./photo-story/backend

    # Optimize static files.
    find ./photo-story/static/*.wasm -exec cp {} ./target/unoptimized.wasm \; -exec wasm-snip --snip-rust-panicking-code {} -o {} \; -exec wasm-opt -Oz {} -o {} \;
    find ./photo-story/static/*.js -exec npx terser {} -c -m --output {} \;
    find ./photo-story/static/*.css -exec npx csso {} --comments none --output {} \;

    # Compress static files.
    npx brotli-cli compress -q 11 --glob ./photo-story/static/*.wasm
    npx brotli-cli compress -q 11 --glob ./photo-story/static/*.js
    npx brotli-cli compress -q 11 --glob ./photo-story/static/*.css

    # Compress assets.
    npx brotli-cli compress -q 11 --glob --bail false ./photo-story/assets/*/*


# Cleans the project.
clean:
    rm -r ./photo-story
    rm -r ./node_modules
    cargo clean
    trunk clean

# Cleans the logs.
clean-logs:
    rm ./logs/*

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

# Run docker container.
docker-run:
    docker run -p 9000:9000 -t photo-story:distroless

# Run docker container on host network.
docker-run-host:
    docker run --network=host -p 9000:9000 -t photo-story:distroless

# Docker compose up.
docker-compose-up:
    docker-compose up

# Docker compose down.
docker-compose-down:
    docker-compose down

# Docker kill all running containers.
docker-kill-all:
    docker kill $(docker ps -qa)

# Expands macro in file and outputs it to console.
expand-macro FILE:
    rustc +nightly -Zunpretty=expanded {{FILE}}

# Cargo and clippy fix.
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Make .githooks this project hooks lookup directory.
init-git-hooks:
    git config --local core.hooksPath .githooks

# Install needed dev dependencies and configurations.
install-init-dev:
    just init-git-hooks
    npm install
    cargo install trunk
    just install-mold-linker
    just install-udeps
    rustup target add wasm32-unknown-unknown

# Install Loki Docker Driver plugin to monitor containers.
install-loki-docker-driver:
    docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions
    sudo cp ./monitoring/loki/daemon.json /etc/docker/daemon.json
    sudo systemctl restart docker

# Uninstall Loki Docker Driver plugin.
uninstall-loki-docker-driver:
    docker plugin disable loki
    docker plugin rm loki
    sudo rm /etc/docker/daemon.json
    sudo systemctl restart docker

# Install mold linker for faster compilation linker.
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

# Convert file in current directory to the webp format using ImageMagick. Recommended: 50 or 80.
magick-img-to-webp FILE QUALITY="50":
    FILENAME={{FILE}} && \
    FILENAME=$(echo "${FILENAME%.*}") && \
    magick {{invocation_directory()}}/{{FILE}} -quality {{QUALITY}} -define webp:method=6 {{invocation_directory()}}/$FILENAME-q{{QUALITY}}.webp

# Convert file in current directory to the avif format using ImageMagick. Recommended: 50 or 75.
magick-img-to-avif FILE QUALITY="50":
    FILENAME={{FILE}} && \
    FILENAME=$(echo "${FILENAME%.*}") && \
    magick {{invocation_directory()}}/{{FILE}} -quality {{QUALITY}} -define heic:speed=2 {{invocation_directory()}}/$FILENAME-q{{QUALITY}}.avif

# Convert file in current directory to the webp format using ImageMagick. Recommended: 50 or 80.
magick-resize FILE WIDTH HEIGHT:
    FILENAME={{FILE}} && \
    EXTENSION="${FILENAME##*.}" && \
    FILENAME=$(echo "${FILENAME%.*}") && \
    magick {{invocation_directory()}}/{{FILE}} -resize {{WIDTH}}x{{HEIGHT}} {{invocation_directory()}}/$FILENAME-{{WIDTH}}x{{HEIGHT}}.$EXTENSION

# Run backend.
run-backend PORT STATIC_DIR ASSETS_DIR DEBUG_FILTER OPTION="":
    BACKEND_GENERAL_RUN_ENV=development cargo run --bin backend {{OPTION}} -- --port {{PORT}} -s {{STATIC_DIR}} --assets-dir {{ASSETS_DIR}} -l {{DEBUG_FILTER}}

# Run both backend and frontend with csr in dev with watch.
run-dev-csr FRONTEND_PORT="5555" BACKEND_PORT="5550" DEBUG_FILTER="info":
    #!/usr/bin/env bash
    mkdir -p ./target/static

    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-serve {{FRONTEND_PORT}} {{BACKEND_PORT}}' & \
    bash -c 'cargo watch -w ./crates/backend -- just run-backend {{BACKEND_PORT}} ./target/static ./assets {{DEBUG_FILTER}} --no-default-features')

# Run both backend and frontend with ssr in dev with watch.
run-dev-ssr BACKEND_PORT="5555" DEBUG_FILTER="info":
    #!/usr/bin/env bash
    mkdir -p ./target/static
    trunk build --features ssr --public-url "/static"

    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-watch ssr /static' & \
    bash -c 'cargo watch -w ./crates -w ./static -- just run-backend {{BACKEND_PORT}} {{justfile_directory()}}/static {{justfile_directory()}}/assets {{DEBUG_FILTER}}')

# Format using custom rustfmt.
rustfmt:
    find -type f -path "./crates/*" -path "*.rs" | xargs ./rustfmt --edition 2021

# Serve frontend.
trunk-serve PORT BACKEND_PORT="5550" RENDER="csr":
    trunk serve --features {{RENDER}} --port {{PORT}} --proxy-backend "http://localhost:{{BACKEND_PORT}}/api"

# Watch frontend.
trunk-watch RENDER="csr" PUBLIC_URL="/":
    trunk watch --features {{RENDER}} --public-url {{PUBLIC_URL}}

# Runs all macros.
test-all:
    cargo test --locked

# Runs macros of the specified package.
test PACKAGE:
    cargo test -p {{PACKAGE}} --locked

# Use udeps to find unused dependencies.
udeps:
    cargo +nightly udeps --all-targets

# Vendor all dependencies locally.
vendor:
    cargo vendor

_grep_toml_config FILE GROUP_ENV CONFIG_VAR:
    grep -A 100 "^\[{{GROUP_ENV}}\]" {{FILE}} | grep -m 1 -oP '^{{CONFIG_VAR}}\s?=\s?"?\K[^"?]+'
