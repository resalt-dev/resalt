FROM rust:1.61

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold build-essential pkg-config libssl-dev mariadb-client libmariadb-dev

RUN cargo install cargo-watch

RUN cargo install diesel_cli --no-default-features --features "mysql"

EXPOSE 8000

WORKDIR /usr/src/app
CMD cargo watch --poll -x run