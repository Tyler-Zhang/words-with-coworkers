defmodule WordsGameSlack.GameSave do
  alias WordsGameSlack.GameSave.{Game, Player}
  alias WordsGameSlack.Repo

  import Ecto.Query, only: [from: 2]

  def get_game(team_id, channel_id, user_id) do
    query =
      from g in Game,
        join: p in Player,
        on: p.game_id == g.id,
        where:
          p.user_id == ^user_id and
            g.channel_id == ^channel_id and
            g.team_id == ^team_id,
        order_by: p.index,
        preload: [players: p]

    query |> Repo.one()
  end

  def player_in_game?(%Game{} = game, player_id) when is_bitstring(player_id) do
    Enum.any?(game.players, fn player -> player.user_id == player_id end)
  end

  def player_idx_in_game(%Game{} = game, user_id) do
    game.players
    |> Enum.find_index(fn p -> p.user_id == user_id end)
  end

  @spec create_new_game([any], any, any) :: any
  def create_new_game(players, team_id, channel_id) do
    # Create the game
    words_game = WordsGameElixir.new_game(length(players))

    # Create players in the order they are passed in
    players =
      players
      |> Enum.with_index()
      |> Enum.map(fn {{id, name}, idx} ->
        Player.changeset(
          %Player{},
          %{index: idx, team_id: team_id, user_id: id, user_name: name}
        )
      end)

    Game.changeset(
      %Game{},
      %{
        channel_id: channel_id,
        team_id: team_id,
        data: WordsGameElixir.serialize(words_game)
      }
    )
    |> Ecto.Changeset.put_assoc(:players, players)
    |> Repo.insert()
  end

  def update(%Game{} = game, %WordsGameElixir{} = new_words_game) do
    game
    |> Ecto.Changeset.change(%{data: WordsGameElixir.serialize(new_words_game)})
    |> Repo.update()
  end

  def delete(%Game{} = game) do
    game |> Repo.delete()
  end
end
