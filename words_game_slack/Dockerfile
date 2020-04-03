FROM tylerzhang/elixir-rust

COPY . /app
WORKDIR /app

ENV MIX_ENV=prod
ENV DATABASE_URL=$DATABASE_URL
ENV SECRET_KEY_BASE=$SECRET_KEY_BASE
ENV PORT=80
EXPOSE 80

RUN mix local.hex --force
RUN mix local.rebar --force
RUN mix deps.get --only prod
RUN mix compile

CMD ["mix", "phx.server"]
