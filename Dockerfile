FROM rust:latest as builder
WORKDIR /user/src/myapp
COPY . .
RUN cargo build
CMD cargo run 16 10 198.168.122.31 0 