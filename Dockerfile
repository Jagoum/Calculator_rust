FROM rust:latest AS build

WORKDIR /usr/calculator

COPY . .

RUN cargo build 

FROM ubuntu:latest

COPY --from=build  /usr/calculator/target /usr/src/calculator

CMD ["/usr/src/calculator"]
