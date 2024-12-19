FROM rust:1.83 as builder
RUN USER=root

RUN mkdir loxley-backend
WORKDIR /loxley-backend
ADD . ./
RUN cargo clean && cargo build --release

FROM debian:bullseye
ARG APP=/user/src/app
RUN mkdir -p {$APP}

# Copy the compiled binaries into the new container.
COPY --from=builder /loxley-backend/target/release/loxley-backend ${APP}/loxley-backend
COPY --from=builder /loxley-backend/Rocket.toml ${APP}/Rocket.toml

WORKDIR ${APP}

EXPOSE 80

CMD ["./loxley-backend"]
