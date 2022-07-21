# SHIP APP
FROM debian:buster-slim

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev

COPY ./target/release/resalt /usr/src/app/resalt

ENV RESALT_FRONTEND_PROXY_ENABLED false
EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt