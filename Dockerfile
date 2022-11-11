# SHIP APP
FROM debian:bookworm-slim

# Upgrade System and Install dependencies
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold libssl-dev libssl-dev mariadb-client libmariadb-dev default-libmysqlclient-dev 

COPY ./resalt /usr/src/app/resalt

ENV RESALT_HTTP_FRONTEND_PROXY_ENABLED false
ENV RUST_LOG_STYLE always
EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt