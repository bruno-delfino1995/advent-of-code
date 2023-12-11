defmodule AdventOfCode.Y2023.D01 do
  use AdventOfCode.Solution

  @impl true
  def basic(input) do
    input
    |> Stream.map(&calibration_value/1)
    |> Enum.reduce(0, &Kernel.+/2)
    |> Integer.to_string()
  end

  defp calibration_value(line) do
    digits = 
      line
      |> String.codepoints()
      |> Enum.filter(&is_digit/1)

    first = List.first(digits, "0")
    last = List.last(digits, "")

    String.to_integer("#{first}#{last}")
  end

  defp is_digit(d), do: d in ~w/0 1 2 3 4 5 6 7 8 9/
end
