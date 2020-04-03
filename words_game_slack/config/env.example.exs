# Example for configuring secrets in development
use Mix.Config

# For slack
config :words_game_slack, :slack_oauth,
  app_id: "",
  client_id: "",
  client_secret: "",
  singing_secret: ""
