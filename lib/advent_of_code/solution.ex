defmodule AdventOfCode.Solution do
  @callback basic(stream :: Enumerable.t()) :: String.t()
  @callback complex(stream :: Enumerable.t()) :: String.t()

  @optional_callbacks complex: 1

  defmacro __using__(_) do
    quote do
      @behaviour AdventOfCode.Solution
      AdventOfCode.Solution.__validate__(__MODULE__)
    end
  end

  def __validate__(module) do
    unless valid?(module) do
      raise ArgumentError, "Invalid module name"
    end
  end

  def index() do
    {:ok, modules} = :application.get_key(:advent_of_code, :modules)

    modules
    |> Enum.filter(&valid?/1)
    |> Enum.map(&with_challenge/1)
    |> Enum.into(%{})
  end

  defp valid?(module) do
    match?(
      ["AdventOfCode", <<"Y", _::binary-size(4)>>, <<"D", _::binary-size(2)>>], 
      Module.split(module)
    )
  end

  defp with_challenge(module) do
    [_, <<"Y", year::binary-size(4)>>, <<"D", day::binary-size(2)>>] = Module.split(module)

    {"#{year}/#{day}", module}
  end
end
