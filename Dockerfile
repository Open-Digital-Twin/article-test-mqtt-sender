FROM rust:latest as builder
WORKDIR /user/src/myapp
COPY . .
RUN cargo build
CMD cargo run 16 10  192.168.1.141 0 