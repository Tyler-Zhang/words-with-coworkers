defmodule WordsGameSlackWeb.Slack.CommandController do
  use WordsGameSlackWeb, :controller
  require Logger
  alias WordsGameSlack.{Commands, GameSave}

  @expected_keys ["channel_id", "command", "text", "response_url", "token", "user_id", "user_name", "team_id"]

  def create(conn, params) do
    if !check_has_keys(params) do
      Logger.error("Params missing expected keys #{params}, expected: #{@expected_keys}")
      nil
    else
      result = Commands.parse(params["command"], params["text"])
                |> execute_command(params)

      respond(conn, result)
    end
  end

  defp execute_command(
    %Commands.Start{players: players},
    %{
      "team_id" => team_id,
      "channel_id" => channel_id,
      "user_id" => user_id,
      "user_name" => user_name
    }
  ) do
    # Check to make sure the this user isn't already in a game
    game = GameSave.get_game(team_id, channel_id, user_id)

    if game == nil do
      new_game = players
        |> List.insert_at(0, {user_id, user_name})
        |> GameSave.create_new_game(team_id, channel_id)

      with {:ok, game} <- new_game do
        Poison.encode!(game)
      end
    else
      "You are already in a game on this channel!"
    end
  end

  defp check_has_keys(params) do
    Enum.all?(@expected_keys, &(Map.has_key?(params, &1)))
  end

  defp respond(conn, data) do
    data = %{"response_type": "in_channel", "text": data }
    json(conn, data)
  end
end

