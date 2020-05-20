# This file has to be built from the root of the project

FROM rust:1.43 as Builder
WORKDIR /usr/data
COPY web/Cargo.toml ./web/Cargo.toml
COPY shared ./shared
RUN mkdir web/src && echo "fn main() {}" >> web/src/main.rs
RUN cd web && cargo build --release
RUN rm -rf web/src
COPY web web/
RUN cd web && cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=Builder  /usr/data/web/target/release/darkforce_web /usr/local/bin/darkforce_web
CMD ["darkforce_web"]
