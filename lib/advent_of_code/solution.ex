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
    unless AdventOfCode.solution?(module) do
      raise ArgumentError, "Invalid module name"
    end
  end
end
