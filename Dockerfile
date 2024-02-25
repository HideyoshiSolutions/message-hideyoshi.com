FROM rust:1.76-buster
LABEL authors="hideyoshi"

COPY . .

RUN cargo build --release
ENTRYPOINT ["target/release/message-hideyoshi-com"]