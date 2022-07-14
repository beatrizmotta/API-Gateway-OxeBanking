FROM rust:1.60 as build

RUN USER=root cargo new --bin apigatewayoxebanking
WORKDIR /apigatewayoxebanking

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rust

COPY ./src ./src

RUN rm ./target/release/deps/apigatewayoxebanking*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build apigatewayoxebanking/target/release/apigatewayoxebanking .
CMD ["./apigatewayoxebanking"]