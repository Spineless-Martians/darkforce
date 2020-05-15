FROM rust:1.43 as Builder
WORKDIR /usr/worker
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" >> src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=Builder  /usr/worker/target/release/darkforce_worker /usr/local/bin/darkforce_worker
CMD ["darkforce_worker"]