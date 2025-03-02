FROM rust:slim

WORKDIR /usr/src/web-giris

COPY . .

RUN cargo build --release

CMD ["./target/release/web-giris"]
