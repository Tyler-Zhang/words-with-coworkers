defmodule WordsGameSlack.Slack do
  alias __MODULE__

  @spec render_game(WordsGameSlack.GameSave.Game.t()) :: {:error, String.t()} | {:ok, String.t()}
  def render_game(game) do
    Slack.Renderer.render_game(game)
  end

  @spec render_help :: String.t()
  def render_help() do
    Slack.Renderer.render_help()
  end

  @spec render_tiles(String.t()) :: String.t()
  def render_tiles(str), do: Slack.Renderer.render_tiles(str)

  @spec render_board(WordsGameSlack.GameSave.Game.t()) :: {:ok, String.t()} | {:error, String.t()}
  def render_board(game) do
    with {:ok, words_game} <- WordsGameElixir.deserialize(game) do
      Slack.Renderer.render_board(words_game.board)
    end
  end

  @spec render_play_word_result(WordsGameElixir.PlayWordResult.t(), String.t()) :: String.t()
  def render_play_word_result(play_word_result, player_name),
    do: Slack.Renderer.render_play_word_result(play_word_result, player_name)
end
