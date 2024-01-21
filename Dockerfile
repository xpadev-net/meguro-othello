FROM rust:1
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
# NOTE: Set `--rev` to the commit you use in your project
RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
    cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --rev bb44d71a7881758423f0067a6c9f8a8b69b93428 --locked

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    /usr/local/cargo/bin/mzoon build -r

RUN --mount=type=cache,target=target \
    ["cp", "./target/release/backend", "/usr/local/bin/moon_app"]

ENTRYPOINT ["moon_app"]