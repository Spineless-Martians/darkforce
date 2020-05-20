# This file has to be built from the root of the project

FROM rust:1.43 as Builder
WORKDIR /usr/data
COPY scheduler/Cargo.toml ./scheduler/Cargo.toml
COPY shared ./shared
RUN mkdir scheduler/src && echo "fn main() {}" >> scheduler/src/main.rs
RUN cd scheduler && cargo build --release
RUN rm -rf scheduler/src
COPY scheduler scheduler/
RUN cd scheduler && cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=Builder  /usr/data/scheduler/target/release/darkforce_scheduler /usr/local/bin/darkforce_scheduler
CMD ["darkforce_scheduler"]
