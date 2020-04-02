defmodule WordsGameSlack.Repo.Migrations.CreatePlayers do
  use Ecto.Migration

  def change do
    create table(:players) do
      add :team_id, :string
      add :user_id, :string
      add :user_name, :string
      add :index, :integer
      add :game_id, references(:games, on_delete: :delete_all), null: false

      timestamps()
    end

    create index(:players, [:game_id])
  end
end
