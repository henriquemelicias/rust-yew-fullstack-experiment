# --------------------------- #
# --------- BUILDER --------- #
# --------------------------- #
FROM rust:latest AS builder

RUN update-ca-certificates

ENV CARGO_TERM_COLOR always

WORKDIR /photo-story
COPY ./ .

# Cache dependencies.
RUN git clone https://github.com/rui314/mold.git \
    && mkdir ./mold/build \
    &&./mold/install-build-deps.sh \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ ./mold/ -B ./mold/build \
    && cmake --build ./mold/build -j $(nproc) \
    && cmake --install ./mold/build

RUN cargo build --release

# -------------------------- #
# --------- IMAGE ---------- #
# -------------------------- #
FROM gcr.io/distroless/cc

WORKDIR /photo-story

# Copy directories.
COPY --from=builder /photo-story/configs ./

# Copy binaries.
COPY --from=builder /photo-story/target/release/backend ./

EXPOSE 9000

CMD ["./backend"]
