#!/usr/bin/env bash

set -e

LIB_DIR=mathfix
TEST_JAVA=java-test


cd $LIB_DIR
cargo build --release

mkdir -p ../$TEST_JAVA/src/main/resources/linux-x86-64
cp target/release/libmathfix.so ../$TEST_JAVA/src/main/resources/linux-x86-64/