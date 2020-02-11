# select build image
FROM rust:1.41.0-stretch as build
# create a new empty shell project
RUN USER=root cargo new --bin api_converter
WORKDIR /api_converter
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs
# copy your source tree
COPY ./src ./src
# build for release
RUN rm ./target/release/deps/api_converter*
RUN cargo build --release

# our final base
FROM debian:stretch
RUN apt-get update && apt-get install -y ca-certificates
# copy the build artifact from the build stage
COPY --from=build /api_converter/target/release/api_converter .
# set the startup command to run your binary
CMD ["./api_converter"]