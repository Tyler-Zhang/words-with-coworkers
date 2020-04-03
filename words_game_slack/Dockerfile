FROM tylerzhang/elixir-rust

COPY . /app
WORKDIR /app

ENV PORT=80
EXPOSE 80

RUN mix local.hex --force
RUN mix local.rebar --force
RUN mix deps.get --only prod

ENV MIX_ENV=prod
RUN DATABASE_URL=""\
    SECRET_KEY_BASE=""\
    mix compile

RUN chmod u+x ./docker-start.sh

ENTRYPOINT [ "./docker-start.sh" ]
CMD ["mix", "phx.server"]
