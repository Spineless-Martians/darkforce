# This file has to be built from the root of the project

FROM rust:1.43 as Builder
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt update && apt install -y yarn
WORKDIR /usr/data
COPY web/Cargo.toml ./web/Cargo.toml
COPY shared ./shared
RUN mkdir web/src && echo "fn main() {}" >> web/src/main.rs
RUN cd web && cargo build --release
RUN rm -f web/target/release/deps/darkforce_web*
RUN rm -rf web/src
COPY web web/
RUN cd web && cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=Builder  /usr/data/web/target/release/darkforce_web /usr/local/bin/darkforce_web
ENV DARKFORCE_WEB_ADDRESS=0.0.0.0:3000
EXPOSE 3000
CMD ["darkforce_web"]
