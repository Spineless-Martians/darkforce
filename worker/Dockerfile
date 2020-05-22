FROM rust:1.43 as Builder
WORKDIR /usr/data
COPY worker/Cargo.toml ./worker/Cargo.toml
RUN mkdir worker/src && echo "fn main() {}" >> worker/src/main.rs
RUN cd worker && cargo build --release
RUN rm -f worker/target/release/deps/darkforce_worker*
RUN rm -rf worker/src
COPY worker ./worker
RUN cd worker && cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=Builder  /usr/data/worker/target/release/darkforce_worker /usr/local/bin/darkforce_worker
CMD ["darkforce_worker"]
