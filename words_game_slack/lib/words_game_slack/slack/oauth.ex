defmodule WordsGameSlack.Slack.Oauth do
  alias WordsGameSlack.Repo
  alias WordsGameSlack.Slack.Team

  @slack_oauth_url "https://slack.com/api/oauth.access"

  def authorize(code) do
    with {:ok, response} <- request_token(code) do
      IO.inspect(response)
      response
        |> Map.get("team_id")
        |> get_team_by_id
        |> upsert_team(response)
    end
  end

  # No team available, we create a new one
  defp upsert_team(nil, data) do
    %Team{}
      |> Team.changeset(data)
      |> Repo.insert
  end

  defp upsert_team(team, data) do
    team
      |> Ecto.Changeset.change(data)
      |> Repo.update
  end

  defp get_team_by_id(team_id) do
    Repo.get_by(Team, team_id: team_id)
  end

  defp request_token(code) do
    payload = [code: code] ++ auth_data()
    IO.inspect(payload)
    HTTPoison.post(@slack_oauth_url, {:form, payload})
  end

  defp auth_data do
    config = Application.get_env(:words_game_slack, :slack_oauth)
    [
      client_id: config[:client_id],
      client_secret: config[:client_secret]
    ]
  end
end

