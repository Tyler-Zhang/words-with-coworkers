# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :words_game_slack,
  ecto_repos: [WordsGameSlack.Repo]

# Configures the endpoint
config :words_game_slack, WordsGameSlackWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "AucW0PDHlOtdlzqPz6MF4AJQumGLhMFGkPOSkaHdI7qy+eqGwV9nRLjt6JcKu6yE",
  render_errors: [view: WordsGameSlackWeb.ErrorView, accepts: ~w(html json)],
  pubsub: [name: WordsGameSlack.PubSub, adapter: Phoenix.PubSub.PG2],
  live_view: [signing_salt: "10T9K6bu"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
