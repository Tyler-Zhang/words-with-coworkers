defmodule WordsGameSlack.Repo.Migrations.CreateGames do
  use Ecto.Migration

  def change do
    create table(:games) do
      add :data, :text
      add :channel_id, :string
      add :team_id, :string

      timestamps()
    end

    create index(:games, [:team_id, :channel_id])
  end
end
