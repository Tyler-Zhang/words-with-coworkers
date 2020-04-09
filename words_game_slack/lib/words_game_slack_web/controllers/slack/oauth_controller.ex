defmodule WordsGameSlackWeb.Slack.OauthController do
  use WordsGameSlackWeb, :controller
  require Logger

  def create(conn, %{"code" => code}) do
    with {:ok, _} <- WordsGameSlack.Slack.oauth_authorize(code) do
      json(conn, "ok")
    end
  end
end
