defmodule AdventOfCode do
  def solve({day, phase}) do

    module =
      AdventOfCode.Solution.index()
      |> Map.get(day)

    case module do
      nil ->
        {:error, "Solution to challenge #{day} not found"}
      mod ->
        do_solve(mod, phase)
    end
  end

  defp do_solve(module, phase) do
    if can_solve?(module, phase) do
      function = get_function(phase)
      stream = IO.stream(:stdio, :line)

      {:ok, apply(module, function, [stream])}
    else
      {:error, "Module #{module} can't solve phase #{phase}"}
    end
  end

  defp can_solve?(module, phase) do
    Code.ensure_loaded!(module)

    function_exported?(module, get_function(phase), 1)
  end

  defp get_function("1"), do: :basic
  defp get_function("2"), do: :complex
end
