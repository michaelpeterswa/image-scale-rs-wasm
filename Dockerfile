# -=-=-=-=-=-=- Setup Rust Build Container -=-=-=-=-=-=-

FROM rust:alpine as rust-setup

SHELL ["/bin/ash", "-eo", "pipefail", "-c"]
RUN apk update && \ 
    apk add --no-cache musl-dev=1.2.3-r0 curl=7.83.1-r2 && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | ash

# -=-=-=-=-=-=- Compile Rust WASM Module -=-=-=-=-=-=-

FROM rust-setup as rust-build

WORKDIR /build

COPY . .

RUN wasm-pack build --release --target web

# -=-=-=-=-=-=- Setup Apache HTTPD -=-=-=-=-=-=-

FROM httpd:2.4 as web-server

COPY --from=rust-build build/pkg /usr/local/apache2/htdocs/pkg
COPY ./frontend/ /usr/local/apache2/htdocs/

RUN echo "AddType application/wasm .wasm" >> /usr/local/apache2/conf/httpd.conf