FROM rust as base

RUN rustup target add x86_64-apple-darwin
