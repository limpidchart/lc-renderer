FROM rust:1 as builder

WORKDIR ./lc-renderer
COPY . .

RUN rustup component add rustfmt
RUN cargo build --release

FROM debian:stable-slim

ARG LC_RENDERER_DIR=/opt/lc-renderer
ARG LC_RENDERER_USER=lc-renderer

RUN apt-get update \
    && apt-get -y upgrade \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd $LC_RENDERER_USER \
    && useradd -g $LC_RENDERER_USER $LC_RENDERER_USER \
    && mkdir -p $LC_RENDERER_DIR

COPY --from=builder /lc-renderer/target/release/lc-renderer $LC_RENDERER_DIR/lc-renderer

RUN chown -R $LC_RENDERER_USER:$LC_RENDERER_USER $LC_RENDERER_DIR

EXPOSE $LC_RENDERER_PORT

ENV LC_RENDERER_ADDR=0.0.0.0:54020

USER $LC_RENDERER_USER
WORKDIR $LC_RENDERER_DIR

ENTRYPOINT ["./lc-renderer"]
CMD []
