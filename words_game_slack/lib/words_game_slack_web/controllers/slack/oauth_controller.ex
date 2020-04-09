defmodule WordsGameSlackWeb.Slack.OauthController do
  use WordsGameSlackWeb, :controller
  require Logger

  def oauth_authorize(conn, %{"code" => code}) do
    case WordsGameSlack.Slack.oauth_authorize(code) do
      {:ok, _} -> redirect(conn, to: "/slack/oauth_complete")
      {:error, message} ->
        Logger.error(message)
        redirect(conn, to: "/slack/oauth_failed")
    end
  end

  def oauth_complete(conn, _) do
    render(conn, "oauth_complete.html")
  end

  def oauth_failed(conn, _) do
    render(conn, "oauth_failed.html")
  end
end
