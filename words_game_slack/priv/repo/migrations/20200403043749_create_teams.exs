defmodule WordsGameSlack.Repo.Migrations.CreateTeams do
  use Ecto.Migration

  def change do
    create table(:teams) do
      add :access_token, :string
      add :scope, :string
      add :team_name, :string
      add :team_id, :string

      timestamps()

    end

    create index(:teams, [:team_id])
  end
end
