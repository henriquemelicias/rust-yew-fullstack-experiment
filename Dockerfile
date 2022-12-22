# --------------------------- #
# --------- BUILDER --------- #
# --------------------------- #
FROM rust:latest AS builder

RUN update-ca-certificates

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
        clang

ENV CARGO_TERM_COLOR always

WORKDIR /photo-story
COPY ./ .

# Cache dependencies.

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
COPY --from=builder /photo-story/target/release/backend ./

EXPOSE 9000

CMD ["./backend"]
