defmodule Client do
  use HTTPoison.Base

  def generate_rust_primes(n), do: get!("http://localhost:8000/generate_primes?number=#{n}")
  def generate_elixir_primes(n), do: get!("http://localhost:8001/generate_primes?number=#{n}")
end
