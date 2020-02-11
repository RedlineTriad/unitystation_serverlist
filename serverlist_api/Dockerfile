# select build image
FROM rust:1.41.0-stretch as build
# create a new empty shell project
RUN USER=root cargo new --bin serverlist_api
WORKDIR /serverlist_api
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs
# copy your source tree
COPY ./src ./src
# build for release
RUN rm ./target/release/deps/serverlist_api*
RUN cargo build --release

# our final base
FROM debian:stretch
# copy the build artifact from the build stage
COPY --from=build /serverlist_api/target/release/serverlist_api .
# set the startup command to run your binary
CMD ["./serverlist_api"]