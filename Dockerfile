FROM rust:1.63.0

WORKDIR /usr/src/rust-fibonacci-generator

COPY . .

RUN cargo install --path .

CMD ["rust-fibonacci-generator"]
