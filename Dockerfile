FROM rust:latest AS build

WORKDIR /usr/calculator

COPY . .

RUN cargo build 

FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=build  /usr/calculator/target /usr/src/calculator

CMD ["/usr/src/calculator"]
