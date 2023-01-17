#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Build backend and frontend for release.
build-release:
    mkdir -p ./photo-story
    mkdir -p ./photo-story/static
    mkdir -p ./photo-story/logs
    cargo build --profile backend-release --bin backend
    trunk build --release ./crates/frontend/index.html --dist ./photo-story/static --public-url /static/
    rm -f ./photo-story/backend
    cp ./target/backend-release/backend ./photo-story/backend
    cp -r ./assets ./photo-story
    cp -r ./configs ./photo-story
    WASM=$(find ./photo-story/static/*.wasm) \
    && cp $WASM ./target/unoptimized.wasm \
    && wasm-snip --snip-rust-panicking-code $WASM -o $WASM \
    && wasm-opt -Oz $WASM -o $WASM
    JS=$(find ./photo-story/static/*.js) && terser $JS -c -m --output $JS
    just _compress_brotli ./photo-story/static


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
run-backend PORT STATIC_DIR ASSETS_DIR DEBUG_FILTER:
    BACKEND_GENERAL_RUN_ENV=development cargo run --bin backend -- --port {{PORT}} -s {{STATIC_DIR}} --assets-dir {{ASSETS_DIR}} -l DEBUG_FILTER

# Run backend in prod environment.
run-backend-prod PORT STATIC_DIR ASSETS_DIR DEBUG_FILTER:
    BACKEND_GENERAL_RUN_ENV=production cargo run --bin backend --release -- --port {{PORT}} -s {{STATIC_DIR}} --assets-dir {{ASSETS_DIR}} -l DEBUG_FILTER

# Run both backend and frontend in dev with watch.
run-dev PORT="5555" STATIC_DIR="./crates/frontend/dist" ASSETS_DIR="./assets" DEBUG_FILTER="info":
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-watch' & \
    bash -c 'cargo watch -- just run-backend {{PORT}} {{STATIC_DIR}} {{ASSETS_DIR}} {{DEBUG_FILTER}}')

# Run both backend and frontend in prod with watch.
run-prod PORT="5555" STATIC_DIR="./crates/frontend/dist" ASSETS_DIR="./assets" DEBUG_FILTER="info":
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'

    (trap 'kill 0' SIGINT; \
    bash -c 'just trunk-watch-prod' & \
    bash -c 'cargo watch -- just run-backend-prod {{PORT}} {{STATIC_DIR}} {{ASSETS_DIR}} {{DEBUG_FILTER}}')

# Format using custom rustfmt.
rustfmt:
    find -type f -path "./crates/*" -path "*.rs" | xargs ./rustfmt --edition 2021

# Serve frontend in a development runtime enviroment.
trunk-watch:
    trunk watch ./crates/frontend/index.html --public-url=/static/

# Serve frontend in a production runtime enviroment.
trunk-watch-prod:
    trunk watch --release ./crates/frontend/index.html --public-url=/static/

# Runs all tests.
test-all:
    cargo test --locked

# Runs tests of the specified package.
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

# Compresses file using gzip with multiple compression levels and chooses best within epsilon range size difference.
#
# FILE: file to compress.
# EPSILON_RANGE: 0.0-1.0, 0.0 is best compression, 1.0 is best speed.
_compress_gzip_file FILE EPSILON_RANGE:
    #!/bin/bash
    INITIAL_SIZE=$(wc -c < {{FILE}} | bc) # get file initial size

    LAST_COMPRESSED_SIZE=$INITIAL_SIZE
    for i in {1..9} # iterate all 9 levels of compression.
    do
        gzip -$i {{FILE}}  -c > {{FILE}}.gz.$i # compress
        COMPRESSED_SIZE=$(wc -c < {{FILE}}.gz.$i | bc) # new compressed file size.

        # Calculate floating arithmetic differences between file sizes.
        DIFF_EPSILON=$(echo "$INITIAL_SIZE * {{EPSILON_RANGE}}" | bc -l)
        DIFF_COMPRESSED_SIZE=$(echo "$LAST_COMPRESSED_SIZE - $COMPRESSED_SIZE" | bc -l)
        DIFF_COMPARE_GT=$(echo "$DIFF_COMPRESSED_SIZE > $DIFF_EPSILON" | bc -l)

        # best compressed file if the difference is greater than EPSILON_RANGE of initial size compared to the last best compressed file.
        if [[ $DIFF_COMPARE_GT == 1 ]]; then
            BEST_SIZE_FILE=$i # this file is now the best compressed file relatively.
            LAST_COMPRESSED_SIZE=$COMPRESSED_SIZE
        fi

    done

    # Remove all compressed files except the best one.
    for i in {1..9}
    do
        if [[ $i == $BEST_SIZE_FILE ]]; then
            mv -f {{FILE}}.gz.$i {{FILE}}.gz
            continue
        fi

        rm {{FILE}}.gz.$i
    done

# Compresses file using brotli with multiple compression levels and chooses best within epsilon range size difference.
#
# FILE: file to compress.
_compress_brotli_file FILE:
    #!/bin/bash
    brotli -q 11 {{FILE}}  -c > {{FILE}}.br # compress


# Compresses files in directory using gzip, chooses best sizes within epsilon range size difference.
#
# DIRECTORY: files to compress directory.
# EPSILON_RANGE: 0.0-1.0, 0.0 is best compression, 1.0 is best speed.
_compress_gzip DIRECTORY EPSILON_RANGE:
    #!/bin/bash
    for FILE in {{DIRECTORY}}/*.js {{DIRECTORY}}/*.wasm {{DIRECTORY}}/*.css
    do
        if [[ -f $FILE  ]]; then
            echo "GZIP Compressing $FILE"
            just _compress_gzip_file $FILE {{EPSILON_RANGE}}
        fi
    done
#
# Compresses files in directory using brotli, chooses best sizes within epsilon range size difference.
#
# DIRECTORY: files to compress directory.
_compress_brotli DIRECTORY:
    #!/bin/bash
    for FILE in {{DIRECTORY}}/*.js {{DIRECTORY}}/*.wasm {{DIRECTORY}}/*.css
    do
        if [[ -f $FILE  ]]; then
            echo "BROTLI Compressing $FILE"
            just _compress_brotli_file $FILE
        fi
    done
