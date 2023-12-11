defmodule Mix.Tasks.Solve do
  use Mix.Task

  @impl Mix.Task
  def run([challenge]) do
    [day, phase] = String.split(challenge, "-")

    case AdventOfCode.solve({day, phase}) do
      {:ok, result} ->
        IO.puts(result)
      {:error, message} ->
        Mix.shell().error(message)
        System.stop(1)

        :error
    end
  end
end
