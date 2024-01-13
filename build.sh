#!/bin/bash

podman clean && docker build --tag=crate-rs:latest -f Dockerfile