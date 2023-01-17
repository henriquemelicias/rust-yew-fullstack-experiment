# --------------------------- #
# --------- BUILDER --------- #
# --------------------------- #
FROM rust:latest AS builder

RUN update-ca-certificates

RUN echo "deb http://ftp.de.debian.org/debian sid main" >> /etc/apt/sources.list
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
        clang \
        binaryen \
        terser \
        bc \
        brotli

ENV CARGO_TERM_COLOR always

WORKDIR /photo-story

# Install mold.
RUN wget https://github.com/rui314/mold/releases/download/v1.7.1/mold-1.7.1-x86_64-linux.tar.gz \
 && tar -xzf mold* \
 && cp -r ./mold*/* /usr/local/

# Install just.
RUN mkdir just && cd ./just \
    && wget -qO- https://github.com/casey/just/releases/download/1.11.0/just-1.11.0-x86_64-unknown-linux-musl.tar.gz | tar -xzf- \
    && cp just /bin/just \
    && cd ..

RUN cargo install trunk
RUN cargo install wasm-snip

COPY ./ .

RUN rustup target add wasm32-unknown-unknown

RUN just build-release

# -------------------------- #
# --------- IMAGE ---------- #
# -------------------------- #
FROM frolvlad/alpine-glibc:glibc-2.34

WORKDIR /photo-story

COPY --from=builder /photo-story/photo-story ./

ENV BACKEND_GENERAL_RUN_ENV=production
ENTRYPOINT ["/photo-story/backend", "--static-dir", "./static"]
