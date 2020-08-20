defmodule BenchTest do
  use ExUnit.Case

  def benchmark() do
    Benchee.run(
      %{
        "rust" => fn n -> Client.generate_rust_primes(n) end,
        "rust/elixir" => fn n -> Client.generate_elixir_primes(n) end
      },
      warmup: 4,
      time: 5,
      inputs: %{
        "small" => Enum.random(0..5000),
        "medium" => Enum.random(10_000..40_000)
      },
      parallel: 1,
      formatters: [
        Benchee.Formatters.Console,
        {Benchee.Formatters.HTML, file: "output/benchmark.html"}
      ]
    )
  end

  test "benchmark", do: benchmark()
end
