defmodule AdventOfCodeTest do
  use ExUnit.Case

  test "to_stream/1 creates a stream of lines from your string" do
    input = 
      """
      a
      b
      c
      d
      """

    result =
      input
      |> AdventOfCode.to_stream()
      |> Enum.into([])

    assert result == ~w/a b c d/
  end
end
