# BUILD ENVIRONMENT
FROM rust:1.62-slim AS resalt-env

EXPOSE 8000
WORKDIR /usr/src/app

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold build-essential pkg-config libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev



# BUILD APP
# Add source code and build
FROM resalt-env AS resalt-build
ADD . /usr/src/app
RUN cargo build --release



# SHIP APP
# We use a Docker multi-stage build here in order that we only take the compiled executable
FROM alpine:3.13 AS resalt-run
COPY --from=resalt-build /usr/src/app/target/release/resalt /usr/src/app/resalt
ENTRYPOINT ./resalt



# DEVELOP APP
# Remember to mount source code to the container as volume!
FROM resalt-env AS resalt-devel
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features "mysql"
RUN cargo install sccache
ENV RUSTC_WRAPPER sccache
CMD cargo watch --poll -x run