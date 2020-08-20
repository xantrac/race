defmodule NativePrimesGenerator do
  use Rustler, otp_app: :race_elixir, crate: "nativeprimesgenerator"

  # When your NIF is loaded, it will override this function.
  def generate_primes(_a), do: :erlang.nif_error(:nif_not_loaded)
end
