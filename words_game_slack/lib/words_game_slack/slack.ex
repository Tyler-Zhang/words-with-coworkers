defmodule WordsGameSlack.Slack do
  alias __MODULE__

  def render_game(game) do
    Slack.Renderer.render_game(game)
  end

  def render_help() do
    Slack.Renderer.render_help()
  end
end
