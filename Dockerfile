ARG BASE=20.04
FROM ubuntu:${BASE}
RUN apt-get -y update && apt-get -y upgrade && apt-get install -y wget curl less vim
WORKDIR /app
COPY target/release/run_forever .
CMD [ "/app/run_forever" ]
