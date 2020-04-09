defmodule WordsGameSlack.Slack.Config do
  def get(key) do
    slack_config = Application.get_env(:words_game_slack, :slack_oauth)
    slack_config[key]
  end
end
