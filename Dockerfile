# --------------------------- #
# --------- BUILDER --------- #
# --------------------------- #
FROM rust:latest AS builder

RUN update-ca-certificates

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
        clang \
        binaryen \
        terser

ENV CARGO_TERM_COLOR always

WORKDIR /photo-story

# Install mold.
RUN wget https://github.com/rui314/mold/releases/download/v1.7.1/mold-1.7.1-x86_64-linux.tar.gz \
 && tar -xzf mold* \
 && cp -r ./mold*/* /usr/local/

RUN mkdir trunk && cd ./trunk \
    && wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- \
    && cp trunk /bin/trunk \
    && cd ..

RUN mkdir just && cd ./just \
    && wget -qO- https://github.com/casey/just/releases/download/1.11.0/just-1.11.0-x86_64-unknown-linux-musl.tar.gz | tar -xzf- \
    && cp just /bin/just \
    && cd ..

RUN cargo install --locked trunk

COPY ./ .

RUN rustup target add wasm32-unknown-unknown

RUN just build-release

# -------------------------- #
# --------- IMAGE ---------- #
# -------------------------- #
FROM gcr.io/distroless/cc

WORKDIR /photo-story

COPY --from=builder /bin/bash /bin/bash
COPY --from=builder /photo-story/photo-story ./

ENV GENERAL_DEFAULT_RUN_ENV=production
CMD ["./backend", "--static-dir", "./static", "-l", "debug"]
