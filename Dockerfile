# Start with a rust alpine image
FROM rust:alpine3.22
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev curl
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release
RUN strip target/release/tcc-back

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.22
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc curl
# copy the binary into the final image
COPY --from=0 /app/target/release/tcc-back .
EXPOSE 8080
# set the binary as entrypoint
ENTRYPOINT ["/tcc-back"]