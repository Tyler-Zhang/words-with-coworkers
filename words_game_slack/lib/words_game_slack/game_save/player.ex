defmodule WordsGameSlack.GameSave.Player do
  use Ecto.Schema
  import Ecto.Changeset

  schema "players" do
    field :index, :integer
    field :team_id, :string
    field :user_id, :string
    field :user_name, :string
    belongs_to :game, WordsGameSlack.GameSave.Game

    timestamps()
  end

  @doc false
  def changeset(player, attrs) do
    player
    |> cast(attrs, [:team_id, :user_id, :user_name, :index, :game_id])
    |> validate_required([:team_id, :user_id, :user_name, :index])
  end
end
