#!/bin/bash

# Create database
mix ecto.migrate

exec "$@"
