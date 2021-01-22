#!/bin/bash

#Set Environment variable BASE_TAG to set the Ubuntu version.

TAG=${BASE_TAG:-20.04}

echo "Building wih base image: $TAG"

cargo build --release

docker build --tag darobs/run_forever:$TAG --build-arg BASE=$TAG .
