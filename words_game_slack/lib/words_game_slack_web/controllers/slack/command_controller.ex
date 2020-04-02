defmodule WordsGameSlackWeb.Slack.CommandController do
  use WordsGameSlackWeb, :controller
  require Logger
  alias WordsGameSlack.{Commands, GameSave}

  @expected_keys [
    "channel_id",
    "command",
    "text",
    "response_url",
    "token",
    "user_id",
    "user_name",
    "team_id"
  ]

  def create(conn, params) do
    if !check_has_keys(params) do
      Logger.error("Params missing expected keys #{params}, expected: #{@expected_keys}")
      nil
    else
      result =
        with {:ok, command} <- Commands.parse(params["command"], params["text"]) do
          execute_command(command, params)
        end

      case result do
        {:ok, data} -> respond(conn, data)
        {:error, reason} -> respond(conn, reason, "ephemeral")
      end
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
      new_game =
        players
        |> List.insert_at(0, {user_id, user_name})
        |> GameSave.create_new_game(team_id, channel_id)

      with {:ok, game} <- new_game do
        WordsGameSlack.Slack.render_game(game)
      end
    else
      {:error, "You are already in a game on this channel!"}
    end
  end

  defp execute_command(%Commands.Help{}, _),
    do: {:ok, WordsGameSlack.Slack.render_help()}

  defp check_has_keys(params) do
    Enum.all?(@expected_keys, &Map.has_key?(params, &1))
  end

  defp respond(conn, data, type \\ "in_channel") do
    data = %{response_type: type, text: data}
    json(conn, data)
  end
end
