FROM rust:1.67

WORKDIR /usr/src/calculator
COPY . .

RUN cargo install --path .

CMD ["calculator"]
