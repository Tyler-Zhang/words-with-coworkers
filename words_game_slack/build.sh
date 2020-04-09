#!/bin/bash

docker build . -t tylerzhang/words_game_slack \
    --build-arg GIT_SHA="$(git rev-parse --short HEAD)"
