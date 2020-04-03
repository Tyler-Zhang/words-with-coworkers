defmodule WordsGameSlackWeb.Router do
  use WordsGameSlackWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", WordsGameSlackWeb do
    pipe_through :browser

    get "/", PageController, :index

    scope "/slack", Slack do
      post "/command", CommandController, :create

      get "/oauth_authorize", OauthController, :create
    end
  end

  # Other scopes may use custom stacks.
  # scope "/api", WordsGameSlackWeb do
  #   pipe_through :api
  # end
end
