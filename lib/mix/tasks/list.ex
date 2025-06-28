defmodule Mix.Tasks.List do
  use Mix.Task

  @impl Mix.Task
  def run([]) do
    AdventOfCode.index()
    |> Enum.map_join("\n", &elem(&1, 0))
    |> IO.puts()
  end
end
