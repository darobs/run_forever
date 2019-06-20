FROM ubuntu:18.04
RUN apt-get update
WORKDIR /app
COPY target/release/run_forever .
CMD [ "/app/run_forever" ]