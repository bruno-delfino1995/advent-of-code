defmodule AdventOfCode.Y2023.D01 do
  use AdventOfCode.Solution

  @impl true
  def basic(input) do
    input
    |> Stream.map(fn line ->
      calibration_value(line, &extract_digits(&1, []))
    end)
    |> Enum.reduce(0, &Kernel.+/2)
    |> Integer.to_string()
  end

  @impl true
  def complex(input) do
    input
    |> Stream.map(fn line ->
      calibration_value(line, &extract_digits(&1, [], :extended))
    end)
    |> Enum.reduce(0, &Kernel.+/2)
    |> Integer.to_string()
  end

  defp calibration_value(line, func) do
    digits = 
      line
      |> then(func)
      |> Enum.reverse()

    first = List.first(digits, 0)
    last = List.last(digits, "")

    String.to_integer("#{first}#{last}")
  end

  defp extract_digits(line, acc, mode \\ :basic)

  @spelled_out ~w/zero one two three four five six seven eight nine/
    |> Enum.map(&{&1, String.last(&1)})
  for {{pattern, last}, number} <- Enum.with_index(@spelled_out) do
    defp extract_digits(<<unquote(pattern), rest::binary>>, digits, :extended) do
      extract_digits("#{unquote(last)}#{rest}", [unquote(number) | digits], :extended)
    end
  end

  @numbers Enum.map(0..9, &{Integer.to_string(&1), &1})
  for {pattern, number} <- @numbers do
    defp extract_digits(<<unquote(pattern), rest::binary>>, digits, mode) do
      extract_digits(rest, [unquote(number) | digits], mode)
    end
  end

  defp extract_digits("", digits, _), do: digits
  defp extract_digits(<<_, rest::binary>>, digits, mode), do: extract_digits(rest, digits, mode)
end
