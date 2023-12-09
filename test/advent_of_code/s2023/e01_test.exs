defmodule AdventOfCode.S2023.E01Test do
  use ExUnit.Case

  alias AdventOfCode.S2023.E01, as: Solution

  test "first example" do
    result =
      """
      1abc2
      pqr3stu8vwx
      a1b2c3d4e5f
      treb7uchet
      """
      |> AdventOfCode.to_stream()
      |> Solution.basic()

    assert result == "142"
  end
end
