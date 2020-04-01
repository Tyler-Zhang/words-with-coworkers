defmodule WordsGameSlackWeb.PageController do
  use WordsGameSlackWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
