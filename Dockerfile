# We are using the rustlang/rust:nightly image because it comes with Cargo watch preinstalled.
# If you want to use the stable Rust version, you will have to install cargo watch manually.
FROM rustlang/rust:nightly

WORKDIR /usr/src/whisper-server
COPY ./whisper-server .

# Install the cargo-watch package.
RUN cargo install cargo-watch

# Build our application.
RUN cargo build --release

CMD ["cargo", "run", "--release"]