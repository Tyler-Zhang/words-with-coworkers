defmodule WordsGameSlack.GameSave.Game do
  use Ecto.Schema
  import Ecto.Changeset

  schema "games" do
    field :channel_id, :string
    field :data, :string
    field :team_id, :string

    has_many :players, WordsGameSlack.GameSave.Player
    timestamps()
  end

  @doc false
  def changeset(game, attrs) do
    game
    |> cast(attrs, [:data, :channel_id, :team_id])
    |> validate_required([:data, :channel_id, :team_id])
  end
end
