defmodule Mix.Tasks.Solve do
  use Mix.Task

  @impl Mix.Task
  def run([challenge]) do
    case AdventOfCode.solve(challenge) do
      {:ok, result} ->
        IO.puts(result)
      {:error, message} ->
        Mix.shell().error(message)
        System.stop(1)

        :error
    end
  end
end
