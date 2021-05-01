# build rust app and copy it into the kaniko container

FROM ekidd/rust-musl-builder AS build
WORKDIR /tmp
COPY Cargo.toml .
COPY src src
RUN cargo build --release
RUN strip /tmp/target/x86_64-unknown-linux-musl/release/kaniko-compose-builder

FROM gcr.io/kaniko-project/executor:debug
COPY --from=build /tmp/target/x86_64-unknown-linux-musl/release/kaniko-compose-builder /usr/local/bin/
ENTRYPOINT []