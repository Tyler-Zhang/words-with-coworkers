defmodule WordsGameSlack.Slack.Team do
  use Ecto.Schema
  import Ecto.Changeset

  schema "teams" do
    field :access_token, :string
    field :scope, :string
    field :team_id, :string
    field :team_name, :string

    timestamps()
  end

  @doc false
  def changeset(team, attrs) do
    team
    |> cast(attrs, [:access_token, :scope, :team_name, :team_id])
    |> validate_required([:access_token, :scope, :team_name, :team_id])
  end
end
