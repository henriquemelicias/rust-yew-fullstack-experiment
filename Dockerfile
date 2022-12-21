# --------------------------- #
# --------- BUILDER --------- #
# --------------------------- #
FROM rust:latest AS builder

RUN update-ca-certificates

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler \
      clang

ENV CARGO_TERM_COLOR always

WORKDIR /photo-story
COPY ./ .
RUN mv ./.cargo/mold /usr/bin/

# Cache dependencies.
RUN apt install -y protobuf-compiler

RUN cargo build --release

# -------------------------- #
# --------- IMAGE ---------- #
# -------------------------- #
FROM gcr.io/distroless/cc

# Import builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /photo-story

# Copy binaries.
COPY --from=builder /photo-story/target/release/monitoring ./

EXPOSE 9000

CMD ["./monitoring"]
