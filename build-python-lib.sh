#!/usr/bin/env bash
set -e

LIB_DIR=mathfix
TEST_PYTHON=python-test

cd $LIB_DIR
cargo build --release

mkdir -p ../$TEST_PYTHON/libs
cp target/release/libmathfix.so ../$TEST_PYTHON/libs/

