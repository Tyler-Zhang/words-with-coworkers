defmodule WordsGameElixir do
  use Rustler, otp_app: :words_game_slack, crate: "words_game_elixir"
  alias __MODULE__

  # When your NIF is loaded, it will override this function.
  def new_game(_player_count), do: :erlang.nif_error(:nif_not_loaded)
  def play_word(
    _game,
    _start,
    _direction,
    _word
  ), do: :erlang.nif_error(:nif_not_loaded)

  @type t :: %WordsGameElixir {
    board: String.t,
    players: Player.t,
    turn: number,
    tile_bag: String.t,
    has_word_been_played: bool
  }
  defstruct [:board, :players, :turn, :tile_bag, :has_word_been_played]

  defmodule Player do
    @type t :: %Player {
      hand: String.t,
      score: number
    }
    defstruct [:hand, :score]
  end

  defmodule Board do
    @type t :: %Board {
      cells: String.t,
      board_dimension: number
    }
    defstruct [:cells, :board_dimension]
  end

  defmodule PlayWordResult do
    @type t :: %PlayWordResult {
      score: number,
      words: [String.t]
    }
    defstruct [:score, :words]
  end

  def serialize(%WordsGameElixir{} = game), do: Poison.encode!(game)

  @spec deserialize(String.t) :: {:error, String.t} | {:ok, WordsGameElixir.t()}
  def deserialize(str) do
    decoded = Poison.decode!(
      str,
      as: %WordsGameElixir{
        board: %Board{},
        players: [%Player{}]
      }
    )

    case decoded do
      nil -> {:error, "Could not serialize"}
      s -> {:ok, s}
    end
  end
end
