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
        {:ok, :ephemeral, data} -> respond(conn, data, true)
        {:ok, data} -> respond(conn, data)
        {:error, reason} -> respond(conn, reason, true)
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
         } = params
       ) do
    # Check to make sure the this user isn't already in a game
    game = game_from_params(params)

    if elem(game, 0) == :error do
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

  defp execute_command(%Commands.Board{}, params) do
    with {:ok, game_save} = game_from_params(params),
         {:ok, rendering} <- WordsGameSlack.Slack.render_game(game_save) do
      {:ok, :ephemeral, rendering}
    end
  end

  defp execute_command(%Commands.Hand{}, %{"user_id" => user_id} = params) do
    with {:ok, game_save} <- game_from_params(params),
         {:ok, words_game} <- WordsGameElixir.deserialize(game_save) do
      player_idx = GameSave.player_idx_in_game(game_save, user_id)

      hand_tiles =
        words_game.players
        |> Enum.at(player_idx)
        |> Map.get(:hand)
        |> WordsGameSlack.Slack.render_tiles()

      {:ok, :ephemeral, "Your hand: #{hand_tiles}"}
    end
  end

  defp execute_command(
         %Commands.Play{start: start, dir: dir, word: word},
         %{"user_id" => user_id} = params
       ) do
    with {:ok, game_save} <- game_from_params(params),
         {:ok, words_game} <- WordsGameElixir.deserialize(game_save),
         :ok <- ensure_player_turn(words_game, GameSave.player_idx_in_game(game_save, user_id)),
         {:ok, play_word_result, new_words_game} <-
           words_game |> WordsGameElixir.play_word(start, dir, word),
         {:ok, new_game_save} <- GameSave.update(game_save, new_words_game),
         {:ok, game_render} <- WordsGameSlack.Slack.render_game(new_game_save) do
      result_render =
        WordsGameSlack.Slack.render_play_word_result(play_word_result, params["user_name"])

      {:ok, "#{result_render}\n#{game_render}"}
    end
  end

  defp execute_command(%Commands.Dict{word: word}, _) do
    is_valid = word |> String.upcase() |> WordsGameElixir.check_dictionary()

    {:ok, :ephemeral, "#{word} is #{if is_valid, do: "valid", else: "not valid"}"}
  end

  defp execute_command(%Commands.Quit{}, params) do
    with {:ok, game} <- game_from_params(params) do
      GameSave.delete(game)
      {:ok, "Game has ended"}
    end
  end

  defp ensure_player_turn(%WordsGameElixir{} = game, player_idx) do
    if WordsGameElixir.get_current_player_idx(game) == player_idx do
      :ok
    else
      {:error, "It is not your turn to play"}
    end
  end

  defp game_from_params(%{"team_id" => team_id, "channel_id" => channel_id, "user_id" => user_id}) do
    case GameSave.get_game(team_id, channel_id, user_id) do
      nil -> {:error, "Game not found"}
      game -> {:ok, game}
    end
  end

  defp check_has_keys(params) do
    Enum.all?(@expected_keys, &Map.has_key?(params, &1))
  end

  defp respond(conn, data, ephemeral \\ false) do
    data = %{
      response_type: if(ephemeral, do: "ephemeral", else: "in_channel"),
      text: data
    }

    json(conn, data)
  end
end
