defmodule WordsGameSlack.Commands do
  alias __MODULE__

  @command_name Application.get_env(:words_game_slack, :command_name)

  def parse(@command_name, text) do
    case String.trim(text) do
      "help" <> rest -> Commands.Help.parse(rest)
      "play" <> rest -> Commands.Play.parse(rest)
      "start" <> rest -> Commands.Start.parse(rest)
    end
  end
  def parse(_, _), do: {:error, "Unsupported command name #{@command_name}"}

  defmodule Help do
    defstruct []

    def parse(text) do
      %Help{}
    end
  end

  defmodule Play do
    defstruct [:start, :dir, :word]

    # Etc /play 7,7 right ACTOR
    #           ^ parse this section
    @regex ~r/^(\d+), ?(\d+) (right|down) (\w+)/

    def parse(text) do
      case Regex.run(@regex, String.trim(text)) do
        nil -> {:error, "command is invalid"}
        [_, start_x, start_y, dir, word] -> %Play{
          start: {start_x, start_y},
          dir: String.downcase(dir),
          word: String.upcase(word)
        }
      end
    end
  end

  defmodule Start do
    @type t :: %Start{
      players: [{String.t, String.t}] # {Id, Name}
    }
    defstruct [:players]

    # etc /start <@URLUY4SCS|me1>, <@URLUY4SCS|me1>
    @regex ~r/<@(\w+)\|(\w+)>/
    def parse(text) do
      players = Regex.scan(@regex, text)

      %Start {
        players: Enum.map(
          players,
          fn ([_, id, name]) -> [id, name] end
        )
      }
    end
  end
end
