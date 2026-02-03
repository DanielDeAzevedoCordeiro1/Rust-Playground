#!/usr/bin/env bash

set -e

DOCKER_IMAGE_NAME=python-mathfix-test-image
CONTAINER_NAME=python-mathfix-test-container

docker build -t $DOCKER_IMAGE_NAME .
docker run -it --rm --name $CONTAINER_NAME $DOCKER_IMAGE_NAME   

#Bash
docker run -it --rm --name $CONTAINER_NAME $DOCKER_IMAGE_NAME bash