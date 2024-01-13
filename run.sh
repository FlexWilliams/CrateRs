#!/bin/bash

docker run -it --rm -p 8000:8000 -e ROCKET_PROFILE='docker' crate-rs