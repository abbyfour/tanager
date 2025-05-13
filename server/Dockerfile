FROM rust:latest

WORKDIR /usr/tanager
COPY . .

RUN cargo install --path .

EXPOSE 2222

CMD ["tanager"]
