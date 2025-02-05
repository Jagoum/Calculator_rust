FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

# Install GCC runtime and other libraries needed by the binary
RUN apk add --no-cache libgcc

COPY --from=build /app/target/release/calculator /app/calculator

CMD [ "/app/calculator" ]
