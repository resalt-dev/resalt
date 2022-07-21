# SHIP APP
FROM debian:buster-slim

COPY ./target/release/resalt /usr/src/app/resalt

ENV RESALT_FRONTEND_PROXY_ENABLED false
EXPOSE 8000
WORKDIR /usr/src/app

CMD /usr/src/app/resalt