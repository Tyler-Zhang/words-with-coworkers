defmodule WordsGameSlackWeb.Slack.CommandController do
  require Logger
  use WordsGameSlackWeb, :controller
  alias WordsGameSlack.Commands

  @expected_keys ["channel_id", "command", "text", "response_url", "token", "user_id", "user_name", "team_id"]

  def create(conn, params) do
    if !check_has_keys(params) do
      Logger.error("Params missing expected keys #{params}, expected: #{@expected_keys}")
      nil
    else
      command = Commands.parse(params["command"], params["text"])
    end
  end

  defp execute_command(
    %Commands.Start{players},
    %{"channel_id": channel_id, "user_id": user_id, "user_name": user_name}
  ) do
    players = players |> List.insert_at(0, [user_id, user_name])
  end

  defp check_has_keys(params) do
    Enum.all?(@expected_keys, &(Map.has_key?(params, &1)))
  end
end
