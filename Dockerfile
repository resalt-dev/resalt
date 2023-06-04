#
# BUILD FRONTEND
#
FROM node:20-alpine as build_frontend
WORKDIR /usr/src/app/frontend
COPY frontend .
RUN npm install && npm run build

#
# BUILD BACKEND
#
FROM rust:1.70-slim as build_backend
WORKDIR /usr/src/app
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold build-essential pkg-config libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev
COPY . .
RUN cargo build --release



#
# SHIP APP
#
FROM debian:bookworm-slim

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold libssl-dev libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev 

# Copy the binary from the build stage
COPY --from=build_backend /usr/src/app/target/release/resalt /usr/src/app/resalt

ENV RUST_LOG_STYLE always

EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt