defmodule WordsGameSlack.Slack.Renderer do
  alias WordsGameSlack.GameSave
  alias WordsGameElixir.{Player,Board}

  def render_game(%GameSave.Game{data: data, players: players}) do
    with {:ok, game_elixir} <- WordsGameElixir.deserialize(data) do
      # Zip together the library's version of player which has hand and score
      # and the GameSave player which contains the name and id
      # player_zip = Enum.zip(game_elixir.players, players)

      {:ok, render_board(game_elixir.board)}
    end
  end

  @spec render_board(Board.t()) :: String.t
  def render_board(%Board{cells: cells, board_dimension: dim}) do
    cells
      |> String.codepoints
      |> Enum.map(&render_tile/1)
      |> Enum.chunk_every(dim)
      |> Enum.map(&Enum.join/1)
      |> Enum.join("\n")
  end

  @spec render_player(Player.t(), bool) :: String.t
  def render_player(%Player{} = player, render_hand \\ false) do
    ""
  end

  @spec render_tile(String.t) :: String.t
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
end
