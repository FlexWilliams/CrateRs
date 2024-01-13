#!/bin/bash

cargo clean && \
docker build --tag=crate-rs:latest -f Dockerfile