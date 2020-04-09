FROM tylerzhang/elixir-rust-node as BUILD

RUN mkdir /app
WORKDIR /app

ARG GIT_SHA
ENV GIT_SHA=${GIT_SHA}

ENV MIX_ENV=prod

# Intall hex and rebar
RUN mix local.hex --force && \
    mix local.rebar --force

# Install mix dependencies
COPY mix.exs mix.lock ./
COPY config config
RUN mix deps.get --only prod
RUN mix deps.compile

# Build assets
COPY assets assets
COPY priv priv
RUN cd assets && npm install && npm run deploy
RUN mix phx.digest

# Build project
COPY native native
COPY lib lib
RUN mix compile
RUN mix release

# Rebuild sentry as it caches files
RUN mix sentry_recompile

# === Release image ===
FROM ubuntu:latest AS APP
RUN apt-get update && apt-get install -y libssl-dev

RUN mkdir /app
WORKDIR /app
COPY --from=build /app/_build/prod/rel/default ./

COPY "docker-start.sh" .
RUN chmod u+x docker-start.sh

ENV HOME=/app
ENV PORT=${PORT:-80}
EXPOSE ${PORT}

ENTRYPOINT [ "./docker-start.sh" ]
CMD ["bin/default", "start"]
