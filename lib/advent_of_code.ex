defmodule AdventOfCode do
  if Mix.env() == :test do
    def to_stream(str) do
      str
      |> String.trim_trailing("\n")
      |> String.splitter("\n")
    end
  else
    def to_stream(path) do
      path
      |> File.stream!()
      |> Stream.map(&String.trim_trailing(&1, "\n"))
    end
  end
end
