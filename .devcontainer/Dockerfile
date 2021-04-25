ARG RUST_ARCH=x86_64-unknown-linux-musl
ARG RUSTUP_URL=https://static.rust-lang.org/rustup/archive/1.23.1/${RUST_ARCH}/rustup-init
ARG RUSTUP_SHA=05c5c05ec76671d73645aac3afbccf2187352fce7e46fc85be859f52a42797f6
ARG RUST_VERSION=1.51.0

FROM alpine AS rustup-init
ARG RUSTUP_URL \
    RUSTUP_SHA

RUN apk add curl --no-cache                               \
 && curl -L -o rustup-init  ${RUSTUP_URL}                 \
 && echo "${RUSTUP_SHA}  rustup-init" | sha256sum -c -    \
 && chmod +x rustup-init


FROM alpine
ARG RUST_ARCH \
    RUST_VERSION

RUN apk add gcc ca-certificates --no-cache
COPY --from=rustup-init rustup-init .

ENV RUSTUP_HOME=/usr/local/rustup            \
    CARGO_HOME=/usr/local/cargo              \
    PATH=/usr/local/cargo/bin:$PATH

RUN ./rustup-init -y --no-modify-path        \
    --profile minimal                        \
    --default-toolchain ${RUST_VERSION}      \
    --default-host ${RUST_ARCH}              \
                                             \
 && chmod -R a+w $RUSTUP_HOME $CARGO_HOME    \
 && rustup --version                         \
 && cargo --version                          \
 && rustc --version

RUN apk add bash alpine-sdk --no-cache