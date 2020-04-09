defmodule WordsGameSlackWeb.Router do
  use WordsGameSlackWeb, :router
  use Plug.ErrorHandler
  use Sentry.Plug

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

      get "/oauth_authorize", OauthController, :oauth_authorize
      get "/oauth_complete", OauthController, :oauth_complete
      get "/oauth_failed", OauthController, :oauth_failed
    end
  end

  # Other scopes may use custom stacks.
  # scope "/api", WordsGameSlackWeb do
  #   pipe_through :api
  # end
end
