defmodule WordsGameSlackWeb.PageView do
  use WordsGameSlackWeb, :view

  def client_id do
    WordsGameSlack.Slack.oauth_config(:client_id)
  end
end
