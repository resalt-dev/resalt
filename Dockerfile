#
# BUILD FRONTEND
#
FROM oven/bun:1.0.14 as build_frontend
WORKDIR /usr/src/app/frontend
COPY frontend .
RUN bun install
RUN bun run build

#
# BUILD BACKEND
#
FROM rust:1.74-slim as build_backend
WORKDIR /usr/src/app
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold build-essential pkg-config libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev
COPY . .
COPY --from=build_frontend /usr/src/app/frontend/build /usr/src/app/frontend/build
RUN cargo build --release



#
# SHIP APP
#
FROM debian:bullseye-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev ca-certificates

# Copy the binary from the build stage
COPY --from=build_backend /usr/src/app/target/release/resalt-server /usr/src/app/resalt-server

ENV RUST_LOG_STYLE always

EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt-server