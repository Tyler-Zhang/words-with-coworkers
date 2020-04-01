defmodule WordsGameSlack.Repo do
  use Ecto.Repo,
    otp_app: :words_game_slack,
    adapter: Ecto.Adapters.Postgres
end
