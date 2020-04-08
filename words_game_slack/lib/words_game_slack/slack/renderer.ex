defmodule WordsGameSlack.Slack.Renderer do
  alias WordsGameSlack.GameSave
  alias WordsGameElixir.{Player, Board, PlayWordResult}

  @command_name Application.get_env(:words_game_slack, :command_name)

  def render_game(%GameSave.Game{data: data, players: players}) do
    with {:ok, game_elixir} <- WordsGameElixir.deserialize(data) do
      board_render = render_board(game_elixir.board)

      # Zip together the library's version of player which has hand and score
      # and the GameSave player which contains the name and id
      player_render =
        Enum.zip(game_elixir.players, players)
        |> Enum.map(&render_player/1)
        |> Enum.join("\n")

      render = "#{board_render}\n#{player_render}"

      {:ok, render}
    end
  end

  @spec render_board(Board.t()) :: String.t()
  def render_board(%Board{cells: cells, board_dimension: dim}) do
    top_row_coords =
      0..(dim - 1)
      |> Enum.map(&render_coordinate_tile/1)
      |> Enum.join()

    top_row = ":scrabble-board:#{top_row_coords}"

    cell_render =
      cells
      |> String.codepoints()
      |> Enum.map(&render_tile/1)
      |> Enum.chunk_every(dim)
      |> Enum.map(&Enum.join/1)
      |> Enum.with_index()
      |> Enum.map(fn {row, idx} -> "#{render_coordinate_tile(idx)}#{row}" end)
      |> Enum.join("\n")

    "#{top_row}\n#{cell_render}"
  end

  @spec render_player({Player.t(), GameSave.Player.t()}) :: String.t()
  def render_player({%Player{score: score}, %GameSave.Player{user_name: name}}) do
    "#{name}: #{score} points"
  end

  @spec render_tiles(String.t()) :: String.t()
  def render_tiles(str) do
    str |> String.codepoints() |> Enum.map(&render_tile/1) |> Enum.join()
  end

  @spec render_tile(String.t()) :: String.t()
  def render_tile(c) do
    tile_name =
      case c do
        x when c >= "A" and c <= "Z" -> "scrabble-#{String.downcase(x)}"
        "2" -> "scrabble-double-word"
        "3" -> "scrabble-triple-word"
        "@" -> "scrabble-double-letter"
        "#" -> "scrabble-triple-letter"
        "+" -> "scrabble-start"
        "." -> "scrabble-board"
      end

    ":#{tile_name}:"
  end

  defp render_coordinate_tile(num) do
    ":#{rem(num, 10) |> num_to_word}:"
  end

  defp num_to_word(num) do
    case num do
      0 -> "zero"
      1 -> "one"
      2 -> "two"
      3 -> "three"
      4 -> "four"
      5 -> "five"
      6 -> "six"
      7 -> "seven"
      8 -> "eight"
      9 -> "nine"
    end
  end

  @spec render_help :: String.t()
  def render_help() do
    ~s"
Here is how you use the Words with Coworkers bot
>`#{@command_name} help` - Brings up this help dialogue
>`#{@command_name} start [tags]` - Start a game with the person you're chatting to

in game:
>`#{@command_name} board` - Show the state of the board
>`#{@command_name} hand` - Shows you your hand
>`#{@command_name} play <word> <x>:<y> <right|down>` - To play a word
>`#{@command_name} dict <word>` - To check if a word is valid
>`#{@command_name} quit` - Quit the current game
    "
  end

  def render_play_word_result(%PlayWordResult{} = result, player_name) do
    ~s"
#{player_name} player the words:
#{result.words |> Enum.join("\n")}
For #{result.score} points
    "
  end
end
