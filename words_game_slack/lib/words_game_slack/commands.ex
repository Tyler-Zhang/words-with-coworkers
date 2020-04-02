defmodule WordsGameSlack.Commands do
  alias __MODULE__

  @command_name Application.get_env(:words_game_slack, :command_name)

  @spec parse(any, any) ::
          {:error, <<_::144, _::_*136>>}
          | {:ok, Commands.Help.t() | Commands.Start.t() | Commands.Play.t()}
  def parse(@command_name, text) do
    case String.trim(text) do
      "help" <> rest -> Commands.Help.parse(rest)
      "play" <> rest -> Commands.Play.parse(rest)
      "start" <> rest -> Commands.Start.parse(rest)
    end
  end

  def parse(_, _), do: {:error, "Unsupported command name #{@command_name}"}

  defmodule Help do
    @type t :: %Help{}
    defstruct []

    @spec parse(any) :: {:ok, WordsGameSlack.Commands.Help.t()}
    def parse(_) do
      {:ok, %Help{}}
    end
  end

  defmodule Play do
    @type t :: %Play{start: {number, number}, dir: String.t(), word: String.t()}
    defstruct [:start, :dir, :word]

    # Etc /play 7,7 right ACTOR
    #           ^ parse this section
    @regex ~r/^(\d+), ?(\d+) (right|down) (\w+)/

    @spec parse(binary) :: {:error, String.t()} | {:ok, WordsGameSlack.Commands.Play.t()}
    def parse(text) do
      case Regex.run(@regex, String.trim(text)) do
        nil ->
          {:error, "command is invalid"}

        [_, start_x, start_y, dir, word] ->
          {:ok, create_play_command(start_x, start_y, dir, word)}
      end
    end

    defp create_play_command(start_x, start_y, dir, word) do
      %Play{
        start: {start_x, start_y},
        dir: String.downcase(dir),
        word: String.upcase(word)
      }
    end
  end

  defmodule Start do
    @type t :: %Start{
            # {Id, Name}
            players: [{String.t(), String.t()}]
          }
    defstruct [:players]

    # etc /start <@URLUY4SCS|me1>, <@URLUY4SCS|me1>
    @regex ~r/<@(\w+)\|(\w+)>/

    @spec parse(String.t()) :: {:ok, WordsGameSlack.Commands.Start.t()}
    def parse(text) do
      players = Regex.scan(@regex, text)

      command = %Start{
        players:
          Enum.map(
            players,
            fn [_, id, name] -> {id, name} end
          )
      }

      {:ok, command}
    end
  end
end
