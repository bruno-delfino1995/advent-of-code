defmodule AdventOfCode do
  def solve(challenge) do
    func = Map.get(index(), challenge)

    case func do
      nil ->
        {:error, "Solution to challenge #{challenge} not found"}
      func ->
        stream = IO.stream(:stdio, :line)

        {:ok, func.(stream)}
    end
  end

  def index() do
    get_modules()
    |> Enum.flat_map(&with_challenge/1)
    |> Enum.into(%{})
  end

  defp get_modules do
    {:ok, modules} = :application.get_key(:advent_of_code, :modules)

    modules
    |> Enum.filter(&solution?/1)
    |> Enum.filter(&Code.ensure_loaded?/1)
  end

  defp with_challenge(module) do
    ["AdventOfCode", <<"Y", year::binary-size(4)>>, <<"D", day::binary-size(2)>>] = Module.split(module)

    basic = if function_exported?(module, :basic, 1), do: [{"y#{year}d#{day}p1", &module.basic/1}], else: []
    complex = if function_exported?(module, :complex, 1), do: [{"y#{year}d#{day}p2", &module.complex/1}], else: []

    basic ++ complex
  end

  def solution?(module) do
    match?(
      ["AdventOfCode", <<"Y", _::binary-size(4)>>, <<"D", _::binary-size(2)>>],
      Module.split(module)
    )
  end
end
