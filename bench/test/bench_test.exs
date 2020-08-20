defmodule BenchTest do
  use ExUnit.Case

  def benchmark(parallel) do
    Benchee.run(
      %{
        "rust" => fn n -> Client.generate_rust_primes(n) end,
        "rust/elixir" => fn n -> Client.generate_elixir_primes(n) end
      },
      warmup: 4,
      time: 10,
      inputs: %{
        "small" => Enum.random(0..5000)
      },
      parallel: parallel,
      formatters: [
        Benchee.Formatters.Console,
        {Benchee.Formatters.HTML, file: "output_parallel_#{parallel}/benchmark_parallel.html"}
      ]
    )
  end

  test "benchmark" do
    benchmark(1)
    benchmark(4)
    benchmark(10)
    benchmark(20)
  end
end
