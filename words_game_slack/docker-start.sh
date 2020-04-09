#!/bin/bash

# Create database
bin/default eval "WordsGameSlack.Release.migrate"

exec "$@"
