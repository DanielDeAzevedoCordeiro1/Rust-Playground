#!/usr/bin/env bash

set -e

docker build -t java-test-image .
docker run -it --rm --name java-test-container java-test-image