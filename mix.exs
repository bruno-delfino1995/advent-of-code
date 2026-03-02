defmodule AdventOfCode.MixProject do
  use Mix.Project

  def project do
    [
      app: :advent_of_code,
      description: "Solutions for Advent of Code in Elixir",
      version: "0.0.0",
      elixir: "~> 1.18",
      deps: deps()
    ]
  end

  def application do
    []
  end

  defp deps do
    []
  end
end
