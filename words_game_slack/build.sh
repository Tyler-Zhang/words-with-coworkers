#!/bin/bash

docker build \
    --build-arg DATABASE_URL=postgres://user:password@localhost:5432/words_game_slack \
    --build-arg SECRET_KEY_BASE=12131kljgklsajfkslajgkh3jrljklafds \
    .
