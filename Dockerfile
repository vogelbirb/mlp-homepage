FROM rust:1.90

WORKDIR /usr/src/mlp-homepage
COPY . .

RUN cargo install --path .

CMD ["mlp-homepage"]
