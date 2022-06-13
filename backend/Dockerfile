# BUILD ENVIRONMENT
FROM rust:1.61-slim AS hibike-env

EXPOSE 8000
WORKDIR /usr/src/app

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold build-essential pkg-config libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev



# BUILD APP
# Add source code and build
FROM hibike-env AS hibike-build
ADD . /usr/src/app
RUN cargo build --release



# SHIP APP
# We use a Docker multi-stage build here in order that we only take the compiled executable
FROM alpine:3.13 AS hibike-run
COPY --from=hibike-build /usr/src/app/target/release/hibike /usr/src/app/hibike
ENTRYPOINT ./hibike



# DEVELOP APP
# Remember to mount source code to the container as volume!
FROM hibike-env AS hibike-devel
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features "mysql"
CMD cargo watch --poll -x run