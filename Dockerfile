# SHIP APP
FROM debian:bullseye-slim

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold libssl-dev libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev
COPY ./docker/libmysqlclient21_8.0.29-0ubuntu0.20.04.3_amd64.deb /tmp/libmysqlclient21_8.0.29-0ubuntu0.20.04.3_amd64.deb
RUN dpkg -i /tmp/libmysqlclient21_8.0.29-0ubuntu0.20.04.3_amd64.deb

COPY ./resalt /usr/src/app/resalt

ENV RESALT_FRONTEND_PROXY_ENABLED false
ENV RUST_LOG_STYLE always
EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt